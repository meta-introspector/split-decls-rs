// Generated macro for impl_382 (impl)
macro_rules! Depcrate_future_joinimpl_382 {
() => {
// Module: crate::future::join
// Provides: {"impl_382"}
// Dependencies: {}
impl < Fut1 : FusedFuture , Fut2 : FusedFuture > FusedFuture for Join < Fut1 , Fut2 > { fn is_terminated (& self) -> bool { self . fut1 . is_terminated () && self . fut2 . is_terminated () } }
};
}
