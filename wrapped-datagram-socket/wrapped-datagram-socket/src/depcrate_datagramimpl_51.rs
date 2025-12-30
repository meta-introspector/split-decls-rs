// Generated macro for impl_51 (impl)
macro_rules! Depcrate_datagramimpl_51 {
() => {
// Module: crate::datagram
// Provides: {"impl_51"}
// Dependencies: {}
# [cfg (unix)] impl DatagramSocketRecv for Arc < UnixDatagram > { # [inline] fn poll_recv (& mut self , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { UnixDatagram :: poll_recv (self , cx , buf) } # [cfg (target_os = "linux")] # [inline] fn poll_recv_many (& mut self , cx : & mut Context < '_ > , bufs : & mut [ReadBuf < '_ >] ,) -> Poll < io :: Result < usize > > { crate :: poll_recvmmsg ! (self , cx , bufs) } }
};
}
