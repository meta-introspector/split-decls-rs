// Generated macro for impl_95 (impl)
macro_rules! Depcrate_remuteximpl_95 {
() => {
// Module: crate::remutex
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > Deref for ReentrantMutexGuard < 'a , R , G , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . remutex . data . get () } } }
};
}
