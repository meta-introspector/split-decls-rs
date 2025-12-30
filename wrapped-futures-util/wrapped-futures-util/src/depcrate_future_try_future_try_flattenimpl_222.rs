// Generated macro for impl_222 (impl)
macro_rules! Depcrate_future_try_future_try_flattenimpl_222 {
() => {
// Module: crate::future::try_future::try_flatten
// Provides: {"impl_222"}
// Dependencies: {}
impl < Fut > FusedFuture for TryFlatten < Fut , Fut :: Ok > where Fut : TryFuture , Fut :: Ok : TryFuture < Error = Fut :: Error > , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
};
}
