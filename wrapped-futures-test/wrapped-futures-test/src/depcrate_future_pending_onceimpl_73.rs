// Generated macro for impl_73 (impl)
macro_rules! Depcrate_future_pending_onceimpl_73 {
() => {
// Module: crate::future::pending_once
// Provides: {"impl_73"}
// Dependencies: {}
impl < Fut : FusedFuture > FusedFuture for PendingOnce < Fut > { fn is_terminated (& self) -> bool { self . polled_before && self . future . is_terminated () } }
};
}
