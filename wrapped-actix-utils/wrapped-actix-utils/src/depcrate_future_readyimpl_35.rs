// Generated macro for impl_35 (impl)
macro_rules! Depcrate_future_readyimpl_35 {
() => {
// Module: crate::future::ready
// Provides: {"impl_35"}
// Dependencies: {}
impl < T > Future for Ready < T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < T > { let val = self . val . take () . expect ("Ready polled after completion") ; Poll :: Ready (val) } }
};
}
