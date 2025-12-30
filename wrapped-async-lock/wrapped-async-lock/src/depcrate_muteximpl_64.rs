// Generated macro for impl_64 (impl)
macro_rules! Depcrate_muteximpl_64 {
() => {
// Module: crate::mutex
// Provides: {"impl_64"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > fmt :: Debug for MutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
