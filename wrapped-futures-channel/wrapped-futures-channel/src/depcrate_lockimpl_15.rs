// Generated macro for impl_15 (impl)
macro_rules! Depcrate_lockimpl_15 {
() => {
// Module: crate::lock
// Provides: {"impl_15"}
// Dependencies: {}
impl < T > Drop for TryLock < '_ , T > { fn drop (& mut self) { self . __ptr . locked . store (false , SeqCst) ; } }
};
}
