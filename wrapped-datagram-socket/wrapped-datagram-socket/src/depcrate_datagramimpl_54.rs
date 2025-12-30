// Generated macro for impl_54 (impl)
macro_rules! Depcrate_datagramimpl_54 {
() => {
// Module: crate::datagram
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : DatagramSocketSend > MaybeConnectedSocket < T > { pub fn new (inner : T) -> Self { Self { peer : inner . peer_addr () , inner , } } # [doc = " Provides access to the wrapped socket, allowing the user to override"] # [doc = " `send_to()` behavior if required."] pub fn inner (& self) -> & T { & self . inner } # [doc = " Consumes `self`, returning the wrapped socket."] pub fn into_inner (self) -> T { self . inner } }
};
}
