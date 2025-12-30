// Generated macro for impl_210 (impl)
macro_rules! Depcrate_future_try_future_into_futureimpl_210 {
() => {
// Module: crate::future::try_future::into_future
// Provides: {"impl_210"}
// Dependencies: {}
impl < Fut : TryFuture + FusedFuture > FusedFuture for IntoFuture < Fut > { fn is_terminated (& self) -> bool { self . future . is_terminated () } }
};
}
