// Generated macro for impl_45 (impl)
macro_rules! Depcrate_datagramimpl_45 {
() => {
// Module: crate::datagram
// Provides: {"impl_45"}
// Dependencies: {}
impl DatagramSocketRecv for UdpSocket { # [inline] fn poll_recv (& mut self , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { UdpSocket :: poll_recv (self , cx , buf) } # [cfg (target_os = "linux")] # [inline] fn poll_recv_many (& mut self , cx : & mut Context < '_ > , bufs : & mut [ReadBuf < '_ >] ,) -> Poll < io :: Result < usize > > { crate :: poll_recvmmsg ! (self , cx , bufs) } fn as_udp_socket (& self) -> Option < & UdpSocket > { Some (self) } }
};
}
