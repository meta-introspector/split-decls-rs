// Generated macro for impl_2653 (impl)
macro_rules! Depcrate_lock_muteximpl_2653 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2653"}
// Dependencies: {}
impl < T : ? Sized , U : ? Sized + fmt :: Debug > fmt :: Debug for MappedMutexGuard < '_ , T , U > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MappedMutexGuard") . field ("value" , & & * * self) . field ("mutex" , & self . mutex) . finish () } }
};
}
