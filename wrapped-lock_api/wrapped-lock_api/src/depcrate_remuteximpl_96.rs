// Generated macro for impl_96 (impl)
macro_rules! Depcrate_remuteximpl_96 {
() => {
// Module: crate::remutex
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > Drop for ReentrantMutexGuard < 'a , R , G , T > { # [inline] fn drop (& mut self) { unsafe { self . remutex . raw . unlock () ; } } }
};
}
