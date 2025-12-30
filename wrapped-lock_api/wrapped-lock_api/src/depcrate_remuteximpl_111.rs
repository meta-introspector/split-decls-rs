// Generated macro for impl_111 (impl)
macro_rules! Depcrate_remuteximpl_111 {
() => {
// Module: crate::remutex
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , G : GetThreadId + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MappedReentrantMutexGuard < 'a , R , G , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
