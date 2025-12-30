// Generated macro for impl_72 (impl)
macro_rules! Depcrate_future_pending_onceimpl_72 {
() => {
// Module: crate::future::pending_once
// Provides: {"impl_72"}
// Dependencies: {}
impl < Fut : Future > Future for PendingOnce < Fut > { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if * this . polled_before { this . future . poll (cx) } else { * this . polled_before = true ; cx . waker () . wake_by_ref () ; Poll :: Pending } } }
};
}
