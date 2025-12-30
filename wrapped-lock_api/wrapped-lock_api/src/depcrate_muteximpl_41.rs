// Generated macro for impl_41 (impl)
macro_rules! Depcrate_muteximpl_41 {
() => {
// Module: crate::mutex
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MutexGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
