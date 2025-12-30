// Generated macro for impl_72 (impl)
macro_rules! Depcrate_muteximpl_72 {
() => {
// Module: crate::mutex
// Provides: {"impl_72"}
// Dependencies: {}
impl < T : ? Sized > Drop for MutexGuardArc < T > { # [inline] fn drop (& mut self) { unsafe { self . 0 . unlock_unchecked () ; } } }
};
}
