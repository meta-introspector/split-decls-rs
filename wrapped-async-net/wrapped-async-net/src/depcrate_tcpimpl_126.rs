// Generated macro for impl_126 (impl)
macro_rules! Depcrate_tcpimpl_126 {
() => {
// Module: crate::tcp
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (unix)] impl AsFd for TcpStream { fn as_fd (& self) -> BorrowedFd < '_ > { self . inner . get_ref () . as_fd () } }
};
}
