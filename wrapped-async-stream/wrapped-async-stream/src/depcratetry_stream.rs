// Generated macro for try_stream (macro)
macro_rules! Depcratetry_stream {
() => {
// Module: crate
// Provides: {"try_stream"}
// Dependencies: {}
# [doc = " Asynchronous fallible stream"] # [doc = ""] # [doc = " See [crate](index.html) documentation for more details."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tokio::net::{TcpListener, TcpStream};"] # [doc = ""] # [doc = " use async_stream::try_stream;"] # [doc = " use futures_core::stream::Stream;"] # [doc = ""] # [doc = " use std::io;"] # [doc = " use std::net::SocketAddr;"] # [doc = ""] # [doc = " fn bind_and_accept(addr: SocketAddr)"] # [doc = "     -> impl Stream<Item = io::Result<TcpStream>>"] # [doc = " {"] # [doc = "     try_stream! {"] # [doc = "         let mut listener = TcpListener::bind(addr).await?;"] # [doc = ""] # [doc = "         loop {"] # [doc = "             let (stream, addr) = listener.accept().await?;"] # [doc = "             println!(\"received on {:?}\", addr);"] # [doc = "             yield stream;"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! try_stream { ($ ($ tt : tt) *) => { $ crate :: __private :: try_stream_inner ! (($ crate) $ ($ tt) *) } }
};
}
