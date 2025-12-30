// Generated macro for impl_41 (impl)
macro_rules! Depcrate_datagramimpl_41 {
() => {
// Module: crate::datagram
// Provides: {"impl_41"}
// Dependencies: {}
impl < T > AsDatagramSocketRecv for & mut T where T : DatagramSocketRecv + Send + ? Sized , { type AsRecv = T ; fn as_datagram_socket_recv (& mut self) -> & mut Self :: AsRecv { self } fn as_shared_datagram_socket_recv (& self) -> & Self :: AsRecv { self } }
};
}
