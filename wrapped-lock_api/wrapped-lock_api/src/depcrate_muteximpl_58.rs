// Generated macro for impl_58 (impl)
macro_rules! Depcrate_muteximpl_58 {
() => {
// Module: crate::mutex
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > Drop for MappedMutexGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . raw . unlock () ; } } }
};
}
