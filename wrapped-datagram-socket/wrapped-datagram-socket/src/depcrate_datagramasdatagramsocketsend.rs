// Generated macro for AsDatagramSocketSend (trait)
macro_rules! Depcrate_datagramAsDatagramSocketSend {
() => {
// Module: crate::datagram
// Provides: {"AsDatagramSocketSend"}
// Dependencies: {}
# [doc = " A convenience method that can be implemented for any type if it wants"] # [doc = " to forward its `DatagramSocketSend` functionality to an inner field/socket."] # [doc = " This automatically derives `DatagramSocketSend`."] pub trait AsDatagramSocketSend { type AsSend : DatagramSocketSend + ? Sized ; fn as_datagram_socket_send (& self) -> & Self :: AsSend ; }
};
}
