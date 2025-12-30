// Generated macro for impl_40 (impl)
macro_rules! Depcrate_muteximpl_40 {
() => {
// Module: crate::mutex
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MutexGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
