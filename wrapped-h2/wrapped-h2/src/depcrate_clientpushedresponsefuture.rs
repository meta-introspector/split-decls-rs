// Generated macro for PushedResponseFuture (struct)
macro_rules! Depcrate_clientPushedResponseFuture {
() => {
// Module: crate::client
// Provides: {"PushedResponseFuture"}
// Dependencies: {}
# [doc = " A future of a pushed HTTP response."] # [doc = ""] # [doc = " We have to differentiate between pushed and non pushed because of the spec"] # [doc = " <https://httpwg.org/specs/rfc7540.html#PUSH_PROMISE>"] # [doc = " > PUSH_PROMISE frames MUST only be sent on a peer-initiated stream"] # [doc = " > that is in either the \"open\" or \"half-closed (remote)\" state."] # [derive (Debug)] # [must_use = "futures do nothing unless polled"] pub struct PushedResponseFuture { inner : ResponseFuture , }
};
}
