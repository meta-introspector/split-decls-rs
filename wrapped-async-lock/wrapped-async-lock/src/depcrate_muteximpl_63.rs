// Generated macro for impl_63 (impl)
macro_rules! Depcrate_muteximpl_63 {
() => {
// Module: crate::mutex
// Provides: {"impl_63"}
// Dependencies: {}
impl < T : ? Sized > Drop for MutexGuard < '_ , T > { # [inline] fn drop (& mut self) { unsafe { self . 0 . unlock_unchecked () ; } } }
};
}
