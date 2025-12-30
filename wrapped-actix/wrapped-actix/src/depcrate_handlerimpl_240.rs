// Generated macro for impl_240 (impl)
macro_rules! Depcrate_handlerimpl_240 {
() => {
// Module: crate::handler
// Provides: {"impl_240"}
// Dependencies: {}
impl < M > OneshotSend < M > for Option < OneshotSender < M > > { fn send (self , msg : M) { if let Some (tx) = self { let _ = tx . send (msg) ; } } }
};
}
