// Generated macro for impl_111 (impl)
macro_rules! Depcrate_tcpimpl_111 {
() => {
// Module: crate::tcp
// Provides: {"impl_111"}
// Dependencies: {}
# [cfg (windows)] impl AsSocket for TcpListener { fn as_socket (& self) -> BorrowedSocket < '_ > { self . inner . get_ref () . as_socket () } }
};
}
