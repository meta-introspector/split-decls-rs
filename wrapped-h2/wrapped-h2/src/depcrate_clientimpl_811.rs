// Generated macro for impl_811 (impl)
macro_rules! Depcrate_clientimpl_811 {
() => {
// Module: crate::client
// Provides: {"impl_811"}
// Dependencies: {}
impl ResponseFuture { # [doc = " Returns the stream ID of the response stream."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the lock on the stream store has been poisoned."] pub fn stream_id (& self) -> crate :: StreamId { crate :: StreamId :: from_internal (self . inner . stream_id ()) } # [doc = " Returns a stream of PushPromises"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If this method has been called before"] # [doc = " or the stream was itself was pushed"] pub fn push_promises (& mut self) -> PushPromises { if self . push_promise_consumed { panic ! ("Reference to push promises stream taken!") ; } self . push_promise_consumed = true ; PushPromises { inner : self . inner . clone () , } } }
};
}
