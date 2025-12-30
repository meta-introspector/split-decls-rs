// Generated macro for TlsAcceptor (struct)
macro_rules! DepcrateTlsAcceptor {
() => {
// Module: crate
// Provides: {"TlsAcceptor"}
// Dependencies: {}
# [doc = " A builder for server-side TLS connections."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use native_tls::{Identity, TlsAcceptor, TlsStream};"] # [doc = " use std::fs::File;"] # [doc = " use std::io::{Read};"] # [doc = " use std::net::{TcpListener, TcpStream};"] # [doc = " use std::sync::Arc;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mut file = File::open(\"identity.pfx\").unwrap();"] # [doc = " let mut identity = vec![];"] # [doc = " file.read_to_end(&mut identity).unwrap();"] # [doc = " let identity = Identity::from_pkcs12(&identity, \"hunter2\").unwrap();"] # [doc = ""] # [doc = " let listener = TcpListener::bind(\"0.0.0.0:8443\").unwrap();"] # [doc = " let acceptor = TlsAcceptor::new(identity).unwrap();"] # [doc = " let acceptor = Arc::new(acceptor);"] # [doc = ""] # [doc = " fn handle_client(stream: TlsStream<TcpStream>) {"] # [doc = "     // ..."] # [doc = " }"] # [doc = ""] # [doc = " for stream in listener.incoming() {"] # [doc = "     match stream {"] # [doc = "         Ok(stream) => {"] # [doc = "             let acceptor = acceptor.clone();"] # [doc = "             thread::spawn(move || {"] # [doc = "                 let stream = acceptor.accept(stream).unwrap();"] # [doc = "                 handle_client(stream);"] # [doc = "             });"] # [doc = "         }"] # [doc = "         Err(e) => { /* connection failed */ }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (Clone)] pub struct TlsAcceptor (imp :: TlsAcceptor) ;
};
}
