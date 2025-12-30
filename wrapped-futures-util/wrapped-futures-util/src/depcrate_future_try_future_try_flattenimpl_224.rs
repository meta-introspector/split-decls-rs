// Generated macro for impl_224 (impl)
macro_rules! Depcrate_future_try_future_try_flattenimpl_224 {
() => {
// Module: crate::future::try_future::try_flatten
// Provides: {"impl_224"}
// Dependencies: {}
impl < Fut > FusedStream for TryFlatten < Fut , Fut :: Ok > where Fut : TryFuture , Fut :: Ok : TryStream < Error = Fut :: Error > , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
};
}
