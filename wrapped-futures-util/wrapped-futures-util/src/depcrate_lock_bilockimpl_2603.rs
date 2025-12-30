// Generated macro for impl_2603 (impl)
macro_rules! Depcrate_lock_bilockimpl_2603 {
() => {
// Module: crate::lock::bilock
// Provides: {"impl_2603"}
// Dependencies: {}
impl < T > Drop for BiLockGuard < '_ , T > { fn drop (& mut self) { self . bilock . unlock () ; } }
};
}
