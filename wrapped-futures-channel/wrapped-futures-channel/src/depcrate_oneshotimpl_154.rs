// Generated macro for impl_154 (impl)
macro_rules! Depcrate_oneshotimpl_154 {
() => {
// Module: crate::oneshot
// Provides: {"impl_154"}
// Dependencies: {}
impl < T > Future for Receiver < T > { type Output = Result < T , Canceled > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < T , Canceled > > { self . inner . recv (cx) } }
};
}
