// Generated macro for impl_2636 (impl)
macro_rules! Depcrate_lock_muteximpl_2636 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2636"}
// Dependencies: {}
impl < T : ? Sized + fmt :: Debug > fmt :: Debug for OwnedMutexGuard < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OwnedMutexGuard") . field ("value" , & & * * self) . field ("mutex" , & self . mutex) . finish () } }
};
}
