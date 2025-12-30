// Generated macro for exclude_from_backups_and_indexing (function)
macro_rules! Depcrate_pathsexclude_from_backups_and_indexing {
() => {
// Module: crate::paths
// Provides: {"exclude_from_backups_and_indexing"}
// Dependencies: {}
# [doc = " Mark an existing directory as excluded from backups and indexing."] # [doc = ""] # [doc = " Errors in marking it are ignored."] pub fn exclude_from_backups_and_indexing (p : impl AsRef < Path >) { let path = p . as_ref () ; exclude_from_backups (path) ; exclude_from_content_indexing (path) ; }
};
}
