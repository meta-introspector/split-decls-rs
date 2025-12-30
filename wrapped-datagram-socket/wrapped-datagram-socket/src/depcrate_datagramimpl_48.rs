// Generated macro for impl_48 (impl)
macro_rules! Depcrate_datagramimpl_48 {
() => {
// Module: crate::datagram
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (unix)] impl DatagramSocket for UnixDatagram { fn as_raw_io (& self) -> Option < BorrowedFd < '_ > > { Some (self . as_fd ()) } fn into_fd (self) -> Option < OwnedFd > { Some (into_owned_fd (self . into_std () . ok () ?)) } }
};
}
