// Generated macro for impl_60 (impl)
macro_rules! Depcrate_muteximpl_60 {
() => {
// Module: crate::mutex
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MappedMutexGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
