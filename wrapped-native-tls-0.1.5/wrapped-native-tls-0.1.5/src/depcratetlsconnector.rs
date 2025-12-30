// Generated macro for TlsConnector (struct)
macro_rules! DepcrateTlsConnector {
() => {
// Module: crate
// Provides: {"TlsConnector"}
// Dependencies: {}
# [doc = " A builder for client-side TLS connections."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use native_tls::TlsConnector;"] # [doc = " use std::io::{Read, Write};"] # [doc = " use std::net::TcpStream;"] # [doc = ""] # [doc = " let connector = TlsConnector::builder().unwrap().build().unwrap();"] # [doc = ""] # [doc = " let stream = TcpStream::connect(\"google.com:443\").unwrap();"] # [doc = " let mut stream = connector.connect(\"google.com\", stream).unwrap();"] # [doc = ""] # [doc = " stream.write_all(b\"GET / HTTP/1.0\\r\\n\\r\\n\").unwrap();"] # [doc = " let mut res = vec![];"] # [doc = " stream.read_to_end(&mut res).unwrap();"] # [doc = " println!(\"{}\", String::from_utf8_lossy(&res));"] # [doc = " ```"] # [derive (Clone)] pub struct TlsConnector (()) ;
};
}
