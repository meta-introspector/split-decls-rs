// Generated macro for impl_235 (impl)
macro_rules! Depcrate_future_try_future_try_flatten_errimpl_235 {
() => {
// Module: crate::future::try_future::try_flatten_err
// Provides: {"impl_235"}
// Dependencies: {}
impl < Fut > FusedFuture for TryFlattenErr < Fut , Fut :: Error > where Fut : TryFuture , Fut :: Error : TryFuture < Ok = Fut :: Ok > , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
};
}
