// Generated macro for impl_37 (impl)
macro_rules! Depcrate_datagramimpl_37 {
() => {
// Module: crate::datagram
// Provides: {"impl_37"}
// Dependencies: {}
impl < T : AsDatagramSocketRecv + Send > DatagramSocketRecv for T { # [inline] fn poll_recv (& mut self , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { self . as_datagram_socket_recv () . poll_recv (cx , buf) } # [inline] fn poll_recv_from (& mut self , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < SocketAddr > > { self . as_datagram_socket_recv () . poll_recv_from (cx , buf) } # [inline] fn poll_recv_many (& mut self , cx : & mut Context < '_ > , bufs : & mut [ReadBuf < '_ >] ,) -> Poll < io :: Result < usize > > { self . as_datagram_socket_recv () . poll_recv_many (cx , bufs) } # [inline] fn as_udp_socket (& self) -> Option < & UdpSocket > { self . as_shared_datagram_socket_recv () . as_udp_socket () } }
};
}
