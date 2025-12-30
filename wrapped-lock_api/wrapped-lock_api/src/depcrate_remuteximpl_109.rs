// Generated macro for impl_109 (impl)
macro_rules! Depcrate_remuteximpl_109 {
() => {
// Module: crate::remutex
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > Deref for MappedReentrantMutexGuard < 'a , R , G , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data } } }
};
}
