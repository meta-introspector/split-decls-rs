// Generated macro for impl_42 (impl)
macro_rules! Depcrate_datagramimpl_42 {
() => {
// Module: crate::datagram
// Provides: {"impl_42"}
// Dependencies: {}
impl < T > AsDatagramSocketRecv for Box < T > where T : DatagramSocketRecv + Send + ? Sized , { type AsRecv = T ; fn as_datagram_socket_recv (& mut self) -> & mut Self :: AsRecv { self } fn as_shared_datagram_socket_recv (& self) -> & Self :: AsRecv { self } }
};
}
