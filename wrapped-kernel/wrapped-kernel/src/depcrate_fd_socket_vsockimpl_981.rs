// Generated macro for impl_981 (impl)
macro_rules! Depcrate_fd_socket_vsockimpl_981 {
() => {
// Module: crate::fd::socket::vsock
// Provides: {"impl_981"}
// Dependencies: {}
impl Drop for Socket { fn drop (& mut self) { let mut guard = VSOCK_MAP . lock () ; guard . remove_socket (self . port) ; } }
};
}
