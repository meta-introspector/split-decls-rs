// Generated macro for impl_352 (impl)
macro_rules! Depcrate_future_readyimpl_352 {
() => {
// Module: crate::future::ready
// Provides: {"impl_352"}
// Dependencies: {}
impl < T > Future for Ready < T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < T > { Poll :: Ready (self . 0 . take () . expect ("Ready polled after completion")) } }
};
}
