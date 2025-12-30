// Generated macro for Error (enum)
macro_rules! Depcrate_object_tree_diff_for_eachError {
() => {
// Module: crate::object::tree::diff::for_each
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error return by methods on the [diff platform][Platform]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Diff (# [from] gix_diff :: tree_with_rewrites :: Error) , # [error ("The user-provided callback failed")] ForEach (# [source] Box < dyn std :: error :: Error + Send + Sync + 'static >) , # [error (transparent)] ResourceCache (# [from] crate :: repository :: diff_resource_cache :: Error) , # [error ("Failure during rename tracking")] RenameTracking (# [from] tracker :: emit :: Error) , }
};
}
