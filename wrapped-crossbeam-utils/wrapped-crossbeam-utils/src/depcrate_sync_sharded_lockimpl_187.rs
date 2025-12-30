// Generated macro for impl_187 (impl)
macro_rules! Depcrate_sync_sharded_lockimpl_187 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"impl_187"}
// Dependencies: {}
impl Drop for Registration { fn drop (& mut self) { let mut indices = thread_indices () . lock () . unwrap () ; indices . mapping . remove (& self . thread_id) ; indices . free_list . push (self . index) ; } }
};
}
