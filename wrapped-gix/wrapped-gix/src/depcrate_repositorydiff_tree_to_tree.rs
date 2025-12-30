// Generated macro for diff_tree_to_tree (module)
macro_rules! Depcrate_repositorydiff_tree_to_tree {
() => {
// Module: crate::repository
// Provides: {"diff_tree_to_tree"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "blob-diff")] pub mod diff_tree_to_tree { # [doc = " The error returned by [Repository::diff_tree_to_tree()](crate::Repository::diff_tree_to_tree())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] DiffOptions (# [from] crate :: diff :: options :: init :: Error) , # [error (transparent)] CreateResourceCache (# [from] super :: diff_resource_cache :: Error) , # [error (transparent)] TreeDiff (# [from] gix_diff :: tree_with_rewrites :: Error) , } }
};
}
