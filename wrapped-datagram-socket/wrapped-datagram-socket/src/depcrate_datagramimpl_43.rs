// Generated macro for impl_43 (impl)
macro_rules! Depcrate_datagramimpl_43 {
() => {
// Module: crate::datagram
// Provides: {"impl_43"}
// Dependencies: {}
impl DatagramSocket for UdpSocket { # [cfg (unix)] fn as_raw_io (& self) -> Option < BorrowedFd < '_ > > { Some (self . as_fd ()) } # [cfg (unix)] fn into_fd (self) -> Option < OwnedFd > { Some (into_owned_fd (self . into_std () . ok () ?)) } }
};
}
