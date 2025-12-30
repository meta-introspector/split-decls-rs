// Generated macro for impl_38 (impl)
macro_rules! Depcrate_datagramimpl_38 {
() => {
// Module: crate::datagram
// Provides: {"impl_38"}
// Dependencies: {}
impl < T > AsDatagramSocketSend for & mut T where T : DatagramSocketSend + Send + ? Sized , { type AsSend = T ; fn as_datagram_socket_send (& self) -> & Self :: AsSend { self } }
};
}
