// Generated macro for impl_25 (impl)
macro_rules! Depcrate_async_await_pendingimpl_25 {
() => {
// Module: crate::async_await::pending
// Provides: {"impl_25"}
// Dependencies: {}
impl Future for PendingOnce { type Output = () ; fn poll (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Self :: Output > { if self . is_ready { Poll :: Ready (()) } else { self . is_ready = true ; Poll :: Pending } } }
};
}
