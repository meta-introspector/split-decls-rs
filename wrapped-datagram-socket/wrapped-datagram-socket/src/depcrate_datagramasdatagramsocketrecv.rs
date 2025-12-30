// Generated macro for AsDatagramSocketRecv (trait)
macro_rules! Depcrate_datagramAsDatagramSocketRecv {
() => {
// Module: crate::datagram
// Provides: {"AsDatagramSocketRecv"}
// Dependencies: {}
# [doc = " A convenience method that can be implemented for any type if it wants"] # [doc = " to forward its `DatagramSocketRecv` functionality to an inner field/socket."] # [doc = " This automatically derives `DatagramSocketRecv`."] pub trait AsDatagramSocketRecv { type AsRecv : DatagramSocketRecv + ? Sized ; fn as_datagram_socket_recv (& mut self) -> & mut Self :: AsRecv ; fn as_shared_datagram_socket_recv (& self) -> & Self :: AsRecv ; }
};
}
