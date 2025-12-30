// Generated macro for impl_339 (impl)
macro_rules! Depcrate_future_poll_immediateimpl_339 {
() => {
// Module: crate::future::poll_immediate
// Provides: {"impl_339"}
// Dependencies: {}
impl < T : Future > FusedFuture for PollImmediate < T > { fn is_terminated (& self) -> bool { self . future . is_none () } }
};
}
