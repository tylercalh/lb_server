use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::thread;

fn main() {
    const HOST: &str = "127.0.0.1:8085";

    // Begin listening on host port.
    let listener = TcpListener::bind(HOST)
        .unwrap();

    // Get server ip information.
    let response = listener
        .local_addr()
        .unwrap()
        .ip();
    
    let ip_bytes = match response {
        IpAddr::V4(response) => response.octets(),
        IpAddr::V6(_) => [0;4],
    };

    // Accept client requests.
    for stream in listener.incoming() {
        
        // Spawn thread for each client request.
        thread::spawn(move || {
            let stream = stream.unwrap();
            handle_connection(stream, ip_bytes);
        });
    }
}

// Read the clients request for some CPU intensive work then send a response.
fn handle_connection(mut stream: TcpStream, ip_bytes: [u8; 4]) {
    // Read the request from the client.
    let mut buf_in = [1];
    stream.read(&mut buf_in).unwrap();
    println!("Request from {}", stream.peer_addr().unwrap());

    // Calculate fibonacci.
    let fib = fib(buf_in[0] as u32);

    // Send a response.
    let mut buf_out: [u8; 4];
    buf_out = ip_bytes;
    stream.write(&mut buf_out).unwrap();
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}