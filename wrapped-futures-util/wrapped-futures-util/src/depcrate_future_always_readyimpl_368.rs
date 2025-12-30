// Generated macro for impl_368 (impl)
macro_rules! Depcrate_future_always_readyimpl_368 {
() => {
// Module: crate::future::always_ready
// Provides: {"impl_368"}
// Dependencies: {}
impl < T , F : Fn () -> T > Future for AlwaysReady < T , F > { type Output = T ; # [inline] fn poll (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < T > { Poll :: Ready (self . 0 ()) } }
};
}
