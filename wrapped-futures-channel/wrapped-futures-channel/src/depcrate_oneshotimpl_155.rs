// Generated macro for impl_155 (impl)
macro_rules! Depcrate_oneshotimpl_155 {
() => {
// Module: crate::oneshot
// Provides: {"impl_155"}
// Dependencies: {}
impl < T > FusedFuture for Receiver < T > { fn is_terminated (& self) -> bool { if self . inner . complete . load (SeqCst) { if let Some (slot) = self . inner . data . try_lock () { if slot . is_some () { return false ; } } true } else { false } } }
};
}
