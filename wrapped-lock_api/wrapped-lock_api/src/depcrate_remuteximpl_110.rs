// Generated macro for impl_110 (impl)
macro_rules! Depcrate_remuteximpl_110 {
() => {
// Module: crate::remutex
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > Drop for MappedReentrantMutexGuard < 'a , R , G , T > { # [inline] fn drop (& mut self) { unsafe { self . raw . unlock () ; } } }
};
}
