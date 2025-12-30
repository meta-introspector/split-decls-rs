// Generated macro for impl_44 (impl)
macro_rules! Depcrate_datagramimpl_44 {
() => {
// Module: crate::datagram
// Provides: {"impl_44"}
// Dependencies: {}
impl DatagramSocketSend for UdpSocket { # [inline] fn poll_send (& self , cx : & mut Context , buf : & [u8]) -> Poll < io :: Result < usize > > { UdpSocket :: poll_send (self , cx , buf) } # [inline] fn poll_send_to (& self , cx : & mut Context , buf : & [u8] , addr : SocketAddr ,) -> Poll < io :: Result < usize > > { UdpSocket :: poll_send_to (self , cx , buf , addr) } # [cfg (target_os = "linux")] # [inline] fn poll_send_many (& self , cx : & mut Context , bufs : & [ReadBuf < '_ >] ,) -> Poll < io :: Result < usize > > { crate :: poll_sendmmsg ! (self , cx , bufs) } fn as_udp_socket (& self) -> Option < & UdpSocket > { Some (self) } fn peer_addr (& self) -> Option < SocketAddr > { self . peer_addr () . ok () } }
};
}
