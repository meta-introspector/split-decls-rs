// Generated macro for impl_49 (impl)
macro_rules! Depcrate_datagramimpl_49 {
() => {
// Module: crate::datagram
// Provides: {"impl_49"}
// Dependencies: {}
# [cfg (unix)] impl DatagramSocketSend for UnixDatagram { # [inline] fn poll_send (& self , cx : & mut Context , buf : & [u8]) -> Poll < io :: Result < usize > > { UnixDatagram :: poll_send (self , cx , buf) } # [inline] fn poll_send_to (& self , _ : & mut Context , _ : & [u8] , _ : SocketAddr ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: Unsupported , "invalid address family" ,))) } # [cfg (target_os = "linux")] # [inline] fn poll_send_many (& self , cx : & mut Context , bufs : & [ReadBuf < '_ >] ,) -> Poll < io :: Result < usize > > { crate :: poll_sendmmsg ! (self , cx , bufs) } }
};
}
