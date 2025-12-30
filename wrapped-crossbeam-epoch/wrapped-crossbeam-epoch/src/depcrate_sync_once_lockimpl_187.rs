// Generated macro for impl_187 (impl)
macro_rules! Depcrate_sync_once_lockimpl_187 {
() => {
// Module: crate::sync::once_lock
// Provides: {"impl_187"}
// Dependencies: {}
impl < T > Drop for OnceLock < T > { fn drop (& mut self) { if self . once . is_completed () { unsafe { self . value . get () . cast :: < T > () . drop_in_place () } ; } } }
};
}
