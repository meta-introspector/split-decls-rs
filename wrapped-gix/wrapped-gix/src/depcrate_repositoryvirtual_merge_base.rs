// Generated macro for virtual_merge_base (module)
macro_rules! Depcrate_repositoryvirtual_merge_base {
() => {
// Module: crate::repository
// Provides: {"virtual_merge_base"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "merge")] pub mod virtual_merge_base { # [doc = " The error returned by [Repository::virtual_merge_base()](crate::Repository::virtual_merge_base())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] OpenCommitGraph (# [from] super :: commit_graph_if_enabled :: Error) , # [error (transparent)] VirtualMergeBase (# [from] super :: virtual_merge_base_with_graph :: Error) , } }
};
}
