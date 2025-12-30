// Generated macro for CommitAutoRollback (struct)
macro_rules! Depcrate_configCommitAutoRollback {
() => {
// Module: crate::config
// Provides: {"CommitAutoRollback"}
// Dependencies: {}
# [doc = " A utility structure created by [`SnapshotMut::commit_auto_rollback()`] that restores the previous configuration on drop."] pub struct CommitAutoRollback < 'repo > { # [doc = " The owning repository."] pub repo : Option < & 'repo mut Repository > , pub (crate) prev_config : crate :: Config , }
};
}
