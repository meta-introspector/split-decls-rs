// Generated macro for impl_97 (impl)
macro_rules! Depcrate_remuteximpl_97 {
() => {
// Module: crate::remutex
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for ReentrantMutexGuard < 'a , R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
