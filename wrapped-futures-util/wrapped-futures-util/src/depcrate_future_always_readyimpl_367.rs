// Generated macro for impl_367 (impl)
macro_rules! Depcrate_future_always_readyimpl_367 {
() => {
// Module: crate::future::always_ready
// Provides: {"impl_367"}
// Dependencies: {}
impl < T , F : Fn () -> T > FusedFuture for AlwaysReady < T , F > { fn is_terminated (& self) -> bool { false } }
};
}
