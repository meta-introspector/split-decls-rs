// Generated macro for Error (enum)
macro_rules! Depcrate_status_tree_indexError {
() => {
// Module: crate::status::tree_index
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [Repository::tree_index_status()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] IndexFromMTree (# [from] crate :: repository :: index_from_tree :: Error) , # [error (transparent)] RewritesConfiguration (# [from] crate :: diff :: new_rewrites :: Error) , # [error ("Could not create diff-cache for similarity checks")] DiffResourceCache (# [from] crate :: repository :: diff_resource_cache :: Error) , # [error (transparent)] TreeIndexDiff (# [from] gix_diff :: index :: Error) , }
};
}
