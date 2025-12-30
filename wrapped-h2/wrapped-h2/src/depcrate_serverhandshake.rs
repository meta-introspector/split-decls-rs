// Generated macro for handshake (function)
macro_rules! Depcrate_serverhandshake {
() => {
// Module: crate::server
// Provides: {"handshake"}
// Dependencies: {}
# [doc = " Creates a new configured HTTP/2 server with default configuration"] # [doc = " values backed by `io`."] # [doc = ""] # [doc = " It is expected that `io` already be in an appropriate state to commence"] # [doc = " the [HTTP/2 handshake]. See [Handshake] for more details."] # [doc = ""] # [doc = " Returns a future which resolves to the [`Connection`] instance once the"] # [doc = " HTTP/2 handshake has been completed. The returned [`Connection`]"] # [doc = " instance will be using default configuration values. Use [`Builder`] to"] # [doc = " customize the configuration values used by a [`Connection`] instance."] # [doc = ""] # [doc = " [HTTP/2 handshake]: http://httpwg.org/specs/rfc7540.html#ConnectionHeader"] # [doc = " [Handshake]: ../index.html#handshake"] # [doc = " [`Connection`]: struct.Connection.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use tokio::io::{AsyncRead, AsyncWrite};"] # [doc = " # use h2::server;"] # [doc = " # use h2::server::*;"] # [doc = " #"] # [doc = " # async fn doc<T: AsyncRead + AsyncWrite + Unpin>(my_io: T)"] # [doc = " # {"] # [doc = " let connection = server::handshake(my_io).await.unwrap();"] # [doc = " // The HTTP/2 handshake has completed, now use `connection` to"] # [doc = " // accept inbound HTTP/2 streams."] # [doc = " # }"] # [doc = " #"] # [doc = " # pub fn main() {}"] # [doc = " ```"] pub fn handshake < T > (io : T) -> Handshake < T , Bytes > where T : AsyncRead + AsyncWrite + Unpin , { Builder :: new () . handshake (io) }
};
}
