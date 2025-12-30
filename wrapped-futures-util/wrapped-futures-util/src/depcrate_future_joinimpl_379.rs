// Generated macro for impl_379 (impl)
macro_rules! Depcrate_future_joinimpl_379 {
() => {
// Module: crate::future::join
// Provides: {"impl_379"}
// Dependencies: {}
impl < Fut1 : Future , Fut2 : Future > Join < Fut1 , Fut2 > { pub (crate) fn new (fut1 : Fut1 , fut2 : Fut2) -> Self { Self { fut1 : maybe_done (fut1) , fut2 : maybe_done (fut2) } } }
};
}
