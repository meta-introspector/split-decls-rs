// Generated macro for SnapshotMut (struct)
macro_rules! Depcrate_configSnapshotMut {
() => {
// Module: crate::config
// Provides: {"SnapshotMut"}
// Dependencies: {}
# [doc = " A platform to access configuration values and modify them in memory, while making them available when this platform is dropped"] # [doc = " as form of auto-commit."] # [doc = " Note that the values will only affect this instance of the parent repository, and not other clones that may exist."] # [doc = ""] # [doc = " Note that these values won't update even if the underlying file(s) change."] # [doc = ""] # [doc = " Use [`forget()`][Self::forget()] to not apply any of the changes."] pub struct SnapshotMut < 'repo > { # [doc = " The owning repository."] pub repo : Option < & 'repo mut Repository > , pub (crate) config : gix_config :: File < 'static > , }
};
}
