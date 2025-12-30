// Generated macro for impl_936 (impl)
macro_rules! Depcrate_fd_socket_tcpimpl_936 {
() => {
// Module: crate::fd::socket::tcp
// Provides: {"impl_936"}
// Dependencies: {}
impl Drop for Socket { fn drop (& mut self) { let _ = block_on (self . close () , None) ; let mut guard = NIC . lock () ; for h in self . handle . iter () { guard . as_nic_mut () . unwrap () . destroy_socket (* h) ; } } }
};
}
