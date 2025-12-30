// Generated macro for impl_71 (impl)
macro_rules! Depcrate_future_pending_onceimpl_71 {
() => {
// Module: crate::future::pending_once
// Provides: {"impl_71"}
// Dependencies: {}
impl < Fut : Future > PendingOnce < Fut > { pub (super) fn new (future : Fut) -> Self { Self { future , polled_before : false } } }
};
}
