// Generated macro for DatagramSocket (trait)
macro_rules! Depcrate_datagramDatagramSocket {
() => {
// Module: crate::datagram
// Provides: {"DatagramSocket"}
// Dependencies: {}
# [doc = " Describes an implementation of a connected datagram socket."] # [doc = ""] # [doc = " Rather than using Socket for datagram-oriented sockets, the DatagramSocket"] # [doc = " trait purposely does not implement AsyncRead/AsyncWrite, which are traits"] # [doc = " with stream semantics. For example, the `AsyncReadExt::read_exact` method"] # [doc = " which issues as many reads as possible to fill the buffer provided."] # [doc = ""] # [doc = " For a similar reason, [`std::net::UdpSocket`] does not implement"] # [doc = " [`io::Read`] nor does [`tokio::net::UdpSocket`] implement"] # [doc = " [`tokio::io::AsyncRead`]."] pub trait DatagramSocket : DatagramSocketSend + DatagramSocketRecv + 'static { # [cfg (unix)] fn as_raw_io (& self) -> Option < BorrowedFd < '_ > > ; # [cfg (unix)] fn into_fd (self) -> Option < OwnedFd > ; fn as_buf_io (& mut self) -> Option < & mut dyn RawPoolBufDatagramIo > { None } }
};
}
