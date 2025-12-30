// Generated macro for impl_816 (impl)
macro_rules! Depcrate_clientimpl_816 {
() => {
// Module: crate::client
// Provides: {"impl_816"}
// Dependencies: {}
impl PushedResponseFuture { # [doc = " Returns the stream ID of the response stream."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the lock on the stream store has been poisoned."] pub fn stream_id (& self) -> crate :: StreamId { self . inner . stream_id () } }
};
}
