// Generated macro for impl_111 (impl)
macro_rules! Depcrate_utilitiesimpl_111 {
() => {
// Module: crate::utilities
// Provides: {"impl_111"}
// Dependencies: {}
impl < T > Drop for OnceLock < T > { # [inline] fn drop (& mut self) { if self . once . is_completed () { unsafe { self . value . get_mut () . assume_init_drop () } ; } } }
};
}
