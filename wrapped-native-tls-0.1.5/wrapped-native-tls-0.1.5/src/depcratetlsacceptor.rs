// Generated macro for TlsAcceptor (struct)
macro_rules! DepcrateTlsAcceptor {
() => {
// Module: crate
// Provides: {"TlsAcceptor"}
// Dependencies: {}
# [doc = " A builder for server-side TLS connections."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use native_tls::{Pkcs12, TlsAcceptor, TlsStream};"] # [doc = " use std::fs::File;"] # [doc = " use std::io::{Read};"] # [doc = " use std::net::{TcpListener, TcpStream};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mut file = File::open(\"identity.pfx\").unwrap();"] # [doc = " let mut pkcs12 = vec![];"] # [doc = " file.read_to_end(&mut pkcs12).unwrap();"] # [doc = " let pkcs12 = Pkcs12::from_der(&pkcs12, \"hunter2\").unwrap();"] # [doc = ""] # [doc = " let listener = TcpListener::bind(\"0.0.0.0:8443\").unwrap();"] # [doc = " let acceptor = TlsAcceptor::builder(pkcs12).unwrap().build().unwrap();"] # [doc = " let acceptor = Arc::new(acceptor);"] # [doc = ""] # [doc = " fn handle_client(stream: TlsStream<TcpStream>) {"] # [doc = "     // ..."] # [doc = " }"] # [doc = ""] # [doc = " for stream in listener.incoming() {"] # [doc = "     match stream {"] # [doc = "         Ok(stream) => {"] # [doc = "             let acceptor = acceptor.clone();"] # [doc = "             thread::spawn(move || {"] # [doc = "                 let stream = acceptor.accept(stream).unwrap();"] # [doc = "                 handle_client(stream);"] # [doc = "             });"] # [doc = "         }"] # [doc = "         Err(e) => { /* connection failed */ }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (Clone)] pub struct TlsAcceptor (()) ;
};
}
