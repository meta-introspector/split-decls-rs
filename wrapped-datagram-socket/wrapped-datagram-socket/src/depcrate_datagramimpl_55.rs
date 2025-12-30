// Generated macro for impl_55 (impl)
macro_rules! Depcrate_datagramimpl_55 {
() => {
// Module: crate::datagram
// Provides: {"impl_55"}
// Dependencies: {}
impl < T : DatagramSocketSend > DatagramSocketSend for MaybeConnectedSocket < T > { # [inline] fn poll_send (& self , cx : & mut Context , buf : & [u8]) -> Poll < io :: Result < usize > > { self . inner . poll_send (cx , buf) } # [inline] fn poll_send_to (& self , cx : & mut Context , buf : & [u8] , addr : SocketAddr ,) -> Poll < io :: Result < usize > > { if let Some (peer) = self . peer { debug_assert_eq ! (peer , addr) ; self . inner . poll_send (cx , buf) } else { self . inner . poll_send_to (cx , buf , addr) } } # [inline] fn poll_send_many (& self , cx : & mut Context , bufs : & [ReadBuf < '_ >] ,) -> Poll < io :: Result < usize > > { self . inner . poll_send_many (cx , bufs) } # [inline] fn as_udp_socket (& self) -> Option < & UdpSocket > { self . inner . as_udp_socket () } # [inline] fn peer_addr (& self) -> Option < SocketAddr > { self . peer } }
};
}
