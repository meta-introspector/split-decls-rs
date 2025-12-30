// Generated macro for impl_150 (impl)
macro_rules! Depcrate_udpimpl_150 {
() => {
// Module: crate::udp
// Provides: {"impl_150"}
// Dependencies: {}
# [cfg (windows)] impl AsSocket for UdpSocket { fn as_socket (& self) -> BorrowedSocket < '_ > { self . inner . get_ref () . as_socket () } }
};
}
