// Generated macro for impl_198 (impl)
macro_rules! Depcrate_readyimpl_198 {
() => {
// Module: crate::ready
// Provides: {"impl_198"}
// Dependencies: {}
impl < T > Future for Ready < T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < T > { let val = self . val . take () . expect ("Ready can not be polled twice.") ; Poll :: Ready (val) } }
};
}
