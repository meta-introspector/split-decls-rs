// Generated macro for impl_108 (impl)
macro_rules! Depcrate_tcpimpl_108 {
() => {
// Module: crate::tcp
// Provides: {"impl_108"}
// Dependencies: {}
# [cfg (unix)] impl AsFd for TcpListener { fn as_fd (& self) -> BorrowedFd < '_ > { self . inner . get_ref () . as_fd () } }
};
}
