// Generated macro for impl_39 (impl)
macro_rules! Depcrate_muteximpl_39 {
() => {
// Module: crate::mutex
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > Drop for MutexGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . mutex . raw . unlock () ; } } }
};
}
