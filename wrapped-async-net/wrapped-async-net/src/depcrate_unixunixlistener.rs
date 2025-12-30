// Generated macro for UnixListener (struct)
macro_rules! Depcrate_unixUnixListener {
() => {
// Module: crate::unix
// Provides: {"UnixListener"}
// Dependencies: {}
# [doc = " A Unix server, listening for connections."] # [doc = ""] # [doc = " After creating a [`UnixListener`] by [`bind`][`UnixListener::bind()`]ing it to an address, it"] # [doc = " listens for incoming connections. These can be accepted by calling"] # [doc = " [`accept()`][`UnixListener::accept()`] or by awaiting items from the async stream of"] # [doc = " [`incoming`][`UnixListener::incoming()`] connections."] # [doc = ""] # [doc = " Cloning a [`UnixListener`] creates another handle to the same socket. The socket will be closed"] # [doc = " when all handles to it are dropped."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_net::unix::UnixListener;"] # [doc = " use futures_lite::prelude::*;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let listener = UnixListener::bind(\"/tmp/socket\")?;"] # [doc = " let mut incoming = listener.incoming();"] # [doc = ""] # [doc = " while let Some(stream) = incoming.next().await {"] # [doc = "     let mut stream = stream?;"] # [doc = "     stream.write_all(b\"hello\").await?;"] # [doc = " }"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct UnixListener { inner : Arc < Async < std :: os :: unix :: net :: UnixListener > > , }
};
}
