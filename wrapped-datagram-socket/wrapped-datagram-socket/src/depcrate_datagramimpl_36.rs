// Generated macro for impl_36 (impl)
macro_rules! Depcrate_datagramimpl_36 {
() => {
// Module: crate::datagram
// Provides: {"impl_36"}
// Dependencies: {}
impl < T : AsDatagramSocketSend + Sync > DatagramSocketSend for T { # [inline] fn poll_send (& self , cx : & mut Context , buf : & [u8]) -> Poll < io :: Result < usize > > { self . as_datagram_socket_send () . poll_send (cx , buf) } # [inline] fn poll_send_to (& self , cx : & mut Context , buf : & [u8] , addr : SocketAddr ,) -> Poll < io :: Result < usize > > { self . as_datagram_socket_send () . poll_send_to (cx , buf , addr) } # [inline] fn poll_send_many (& self , cx : & mut Context , bufs : & [ReadBuf < '_ >] ,) -> Poll < io :: Result < usize > > { self . as_datagram_socket_send () . poll_send_many (cx , bufs) } # [inline] fn as_udp_socket (& self) -> Option < & UdpSocket > { self . as_datagram_socket_send () . as_udp_socket () } # [inline] fn peer_addr (& self) -> Option < SocketAddr > { self . as_datagram_socket_send () . peer_addr () } }
};
}
