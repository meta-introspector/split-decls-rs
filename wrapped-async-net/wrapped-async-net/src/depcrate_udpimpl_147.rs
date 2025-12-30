// Generated macro for impl_147 (impl)
macro_rules! Depcrate_udpimpl_147 {
() => {
// Module: crate::udp
// Provides: {"impl_147"}
// Dependencies: {}
# [cfg (unix)] impl AsFd for UdpSocket { fn as_fd (& self) -> BorrowedFd < '_ > { self . inner . get_ref () . as_fd () } }
};
}
