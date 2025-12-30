// Generated macro for impl_46 (impl)
macro_rules! Depcrate_datagramimpl_46 {
() => {
// Module: crate::datagram
// Provides: {"impl_46"}
// Dependencies: {}
impl DatagramSocket for Arc < UdpSocket > { # [cfg (unix)] fn as_raw_io (& self) -> Option < BorrowedFd < '_ > > { Some (self . as_fd ()) } # [cfg (unix)] fn into_fd (self) -> Option < OwnedFd > { None } }
};
}
