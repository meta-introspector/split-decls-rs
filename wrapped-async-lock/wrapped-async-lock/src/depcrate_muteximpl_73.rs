// Generated macro for impl_73 (impl)
macro_rules! Depcrate_muteximpl_73 {
() => {
// Module: crate::mutex
// Provides: {"impl_73"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > fmt :: Debug for MutexGuardArc < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
