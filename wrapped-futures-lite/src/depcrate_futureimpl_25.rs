// Generated macro for impl_25 (impl)
macro_rules! Depcrate_futureimpl_25 {
() => {
// Module: crate::future
// Provides: {"impl_25"}
// Dependencies: {}
impl Future for YieldNow { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { if ! self . 0 { self . 0 = true ; cx . waker () . wake_by_ref () ; Poll :: Pending } else { Poll :: Ready (()) } } }
};
}
