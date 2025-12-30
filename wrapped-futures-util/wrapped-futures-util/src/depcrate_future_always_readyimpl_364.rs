// Generated macro for impl_364 (impl)
macro_rules! Depcrate_future_always_readyimpl_364 {
() => {
// Module: crate::future::always_ready
// Provides: {"impl_364"}
// Dependencies: {}
impl < T , F : Fn () -> T + Clone > Clone for AlwaysReady < T , F > { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
};
}
