// Generated macro for impl_39 (impl)
macro_rules! Depcrate_datagramimpl_39 {
() => {
// Module: crate::datagram
// Provides: {"impl_39"}
// Dependencies: {}
impl < T > AsDatagramSocketSend for Box < T > where T : DatagramSocketSend + Send + ? Sized , { type AsSend = T ; fn as_datagram_socket_send (& self) -> & Self :: AsSend { self } }
};
}
