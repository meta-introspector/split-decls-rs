// Generated macro for impl_2654 (impl)
macro_rules! Depcrate_lock_muteximpl_2654 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2654"}
// Dependencies: {}
impl < T : ? Sized , U : ? Sized > Drop for MappedMutexGuard < '_ , T , U > { fn drop (& mut self) { self . mutex . unlock () } }
};
}
