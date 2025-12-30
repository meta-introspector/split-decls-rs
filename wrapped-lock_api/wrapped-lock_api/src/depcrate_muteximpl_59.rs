// Generated macro for impl_59 (impl)
macro_rules! Depcrate_muteximpl_59 {
() => {
// Module: crate::mutex
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MappedMutexGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
