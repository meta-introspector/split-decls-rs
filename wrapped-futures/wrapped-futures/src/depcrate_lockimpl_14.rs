// Generated macro for impl_14 (impl)
macro_rules! Depcrate_lockimpl_14 {
() => {
// Module: crate::lock
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'a , T > Drop for TryLock < 'a , T > { fn drop (& mut self) { self . __ptr . locked . store (false , Release) ; } }
};
}
