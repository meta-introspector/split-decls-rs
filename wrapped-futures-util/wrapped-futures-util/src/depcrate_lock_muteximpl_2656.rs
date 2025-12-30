// Generated macro for impl_2656 (impl)
macro_rules! Depcrate_lock_muteximpl_2656 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2656"}
// Dependencies: {}
impl < T : ? Sized , U : ? Sized > DerefMut for MappedMutexGuard < '_ , T , U > { fn deref_mut (& mut self) -> & mut U { unsafe { & mut * self . value } } }
};
}
