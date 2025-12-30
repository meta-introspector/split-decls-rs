// Generated macro for impl_1077 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1077 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1077"}
// Dependencies: {}
impl WrappedWaker { # [doc = " Replaces given waker's inner_waker for polling stream/futures which will"] # [doc = " update poll state on `wake_by_ref` call. Use only if you need several"] # [doc = " contexts."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " This function will modify waker's `inner_waker` via `UnsafeCell`, so"] # [doc = " it should be used only during `POLLING` phase by one thread at the time."] unsafe fn replace_waker (self_arc : & mut Arc < Self > , cx : & Context < '_ >) { unsafe { * self_arc . inner_waker . get () = cx . waker () . clone () . into () } } # [doc = " Attempts to start the waking process for the waker with the given value."] # [doc = " If succeeded, then the stream isn't yet woken and not being polled at the moment."] fn start_waking (& self) -> Option < (u8 , PollStateBomb < '_ , impl FnOnce (& SharedPollState) -> u8 >) > { self . poll_state . start_waking (self . need_to_poll) } }
};
}
