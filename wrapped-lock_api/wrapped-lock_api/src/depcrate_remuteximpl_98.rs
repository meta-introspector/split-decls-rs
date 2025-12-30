// Generated macro for impl_98 (impl)
macro_rules! Depcrate_remuteximpl_98 {
() => {
// Module: crate::remutex
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for ReentrantMutexGuard < 'a , R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
