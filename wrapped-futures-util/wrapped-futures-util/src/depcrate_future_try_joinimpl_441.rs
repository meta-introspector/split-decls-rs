// Generated macro for impl_441 (impl)
macro_rules! Depcrate_future_try_joinimpl_441 {
() => {
// Module: crate::future::try_join
// Provides: {"impl_441"}
// Dependencies: {}
impl < Fut1 : TryFuture , Fut2 : TryFuture > TryJoin < Fut1 , Fut2 > { pub (crate) fn new (fut1 : Fut1 , fut2 : Fut2) -> Self { Self { fut1 : try_maybe_done (fut1) , fut2 : try_maybe_done (fut2) } } }
};
}
