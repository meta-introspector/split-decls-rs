// Generated macro for impl_149 (impl)
macro_rules! Depcrate_oneshotimpl_149 {
() => {
// Module: crate::oneshot
// Provides: {"impl_149"}
// Dependencies: {}
impl < T > Future for Cancellation < '_ , T > { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { self . inner . poll_canceled (cx) } }
};
}
