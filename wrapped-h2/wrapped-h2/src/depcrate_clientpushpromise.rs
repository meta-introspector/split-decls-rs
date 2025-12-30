// Generated macro for PushPromise (struct)
macro_rules! Depcrate_clientPushPromise {
() => {
// Module: crate::client
// Provides: {"PushPromise"}
// Dependencies: {}
# [doc = " A pushed response and corresponding request headers"] # [derive (Debug)] pub struct PushPromise { # [doc = " The request headers"] request : Request < () > , # [doc = " The pushed response"] response : PushedResponseFuture , }
};
}
