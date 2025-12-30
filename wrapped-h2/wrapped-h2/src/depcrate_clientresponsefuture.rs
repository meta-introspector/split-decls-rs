// Generated macro for ResponseFuture (struct)
macro_rules! Depcrate_clientResponseFuture {
() => {
// Module: crate::client
// Provides: {"ResponseFuture"}
// Dependencies: {}
# [doc = " A future of an HTTP response."] # [derive (Debug)] # [must_use = "futures do nothing unless polled"] pub struct ResponseFuture { inner : proto :: OpaqueStreamRef , push_promise_consumed : bool , }
};
}
