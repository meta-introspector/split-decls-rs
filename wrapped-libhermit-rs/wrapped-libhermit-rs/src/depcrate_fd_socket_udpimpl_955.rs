// Generated macro for impl_955 (impl)
macro_rules! Depcrate_fd_socket_udpimpl_955 {
() => {
// Module: crate::fd::socket::udp
// Provides: {"impl_955"}
// Dependencies: {}
impl Drop for Socket { fn drop (& mut self) { let _ = block_on (self . close () , None) ; NIC . lock () . as_nic_mut () . unwrap () . destroy_socket (self . handle) ; } }
};
}
