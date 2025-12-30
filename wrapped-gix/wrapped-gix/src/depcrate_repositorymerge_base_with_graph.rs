// Generated macro for merge_base_with_graph (module)
macro_rules! Depcrate_repositorymerge_base_with_graph {
() => {
// Module: crate::repository
// Provides: {"merge_base_with_graph"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "revision")] pub mod merge_base_with_graph { # [doc = " The error returned by [Repository::merge_base_with_cache()](crate::Repository::merge_base_with_graph())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FindMergeBase (# [from] gix_revision :: merge_base :: Error) , # [error ("Could not find a merge-base between commits {first} and {second}")] NotFound { first : gix_hash :: ObjectId , second : gix_hash :: ObjectId , } , } }
};
}
