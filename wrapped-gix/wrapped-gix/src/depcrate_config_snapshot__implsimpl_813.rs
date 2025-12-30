// Generated macro for impl_813 (impl)
macro_rules! Depcrate_config_snapshot__implsimpl_813 {
() => {
// Module: crate::config::snapshot::_impls
// Provides: {"impl_813"}
// Dependencies: {}
impl Drop for CommitAutoRollback < '_ > { fn drop (& mut self) { if let Some (repo) = self . repo . take () { self . rollback_inner (repo) . ok () ; } } }
};
}
