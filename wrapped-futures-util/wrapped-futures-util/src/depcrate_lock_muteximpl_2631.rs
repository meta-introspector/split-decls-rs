// Generated macro for impl_2631 (impl)
macro_rules! Depcrate_lock_muteximpl_2631 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2631"}
// Dependencies: {}
impl < T : ? Sized > fmt :: Debug for OwnedMutexLockFuture < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OwnedMutexLockFuture") . field ("was_acquired" , & self . mutex . is_none ()) . field ("mutex" , & self . mutex) . field ("wait_key" , & (if self . wait_key == WAIT_KEY_NONE { None } else { Some (self . wait_key) }) ,) . finish () } }
};
}
