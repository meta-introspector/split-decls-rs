// Generated macro for impl_112 (impl)
macro_rules! Depcrate_remuteximpl_112 {
() => {
// Module: crate::remutex
// Provides: {"impl_112"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MappedReentrantMutexGuard < 'a , R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
