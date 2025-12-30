// Generated macro for impl_826 (impl)
macro_rules! Depcrate_config_snapshot_accessimpl_826 {
() => {
// Module: crate::config::snapshot::access
// Provides: {"impl_826"}
// Dependencies: {}
# [doc = " Utilities"] impl < 'repo > CommitAutoRollback < 'repo > { # [doc = " Rollback the changes previously applied and all values before the change."] pub fn rollback (mut self) -> Result < & 'repo mut crate :: Repository , crate :: config :: Error > { let repo = self . repo . take () . expect ("still present, consumed only once") ; self . rollback_inner (repo) } pub (crate) fn rollback_inner (& mut self , repo : & 'repo mut crate :: Repository ,) -> Result < & 'repo mut crate :: Repository , crate :: config :: Error > { repo . reread_values_and_clear_caches_replacing_config (OwnShared :: clone (& self . prev_config)) ? ; Ok (repo) } }
};
}
