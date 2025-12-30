// Generated macro for impl_812 (impl)
macro_rules! Depcrate_config_snapshot__implsimpl_812 {
() => {
// Module: crate::config::snapshot::_impls
// Provides: {"impl_812"}
// Dependencies: {}
impl Drop for SnapshotMut < '_ > { fn drop (& mut self) { if let Some (repo) = self . repo . take () { self . commit_inner (repo) . ok () ; } } }
};
}
