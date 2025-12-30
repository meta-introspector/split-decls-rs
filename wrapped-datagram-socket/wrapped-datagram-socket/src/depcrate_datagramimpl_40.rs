// Generated macro for impl_40 (impl)
macro_rules! Depcrate_datagramimpl_40 {
() => {
// Module: crate::datagram
// Provides: {"impl_40"}
// Dependencies: {}
impl < T > AsDatagramSocketSend for Arc < T > where T : DatagramSocketSend + Send + ? Sized , { type AsSend = T ; fn as_datagram_socket_send (& self) -> & Self :: AsSend { self } }
};
}
