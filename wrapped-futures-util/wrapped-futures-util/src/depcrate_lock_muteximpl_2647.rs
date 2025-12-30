// Generated macro for impl_2647 (impl)
macro_rules! Depcrate_lock_muteximpl_2647 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2647"}
// Dependencies: {}
impl < T : ? Sized + fmt :: Debug > fmt :: Debug for MutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MutexGuard") . field ("value" , & & * * self) . field ("mutex" , & self . mutex) . finish () } }
};
}
