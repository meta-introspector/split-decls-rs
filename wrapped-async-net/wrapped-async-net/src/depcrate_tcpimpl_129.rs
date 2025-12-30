// Generated macro for impl_129 (impl)
macro_rules! Depcrate_tcpimpl_129 {
() => {
// Module: crate::tcp
// Provides: {"impl_129"}
// Dependencies: {}
# [cfg (windows)] impl AsSocket for TcpStream { fn as_socket (& self) -> BorrowedSocket < '_ > { self . inner . get_ref () . as_socket () } }
};
}
