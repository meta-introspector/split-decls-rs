// Generated macro for impl_198 (impl)
macro_rules! Depcrate_sync_queueimpl_198 {
() => {
// Module: crate::sync::queue
// Provides: {"impl_198"}
// Dependencies: {}
impl < T > Drop for Queue < T > { fn drop (& mut self) { unsafe { let guard = unprotected () ; while self . try_pop (guard) . is_some () { } let sentinel = self . head . load (Relaxed , guard) ; drop (sentinel . into_owned ()) ; } } }
};
}
