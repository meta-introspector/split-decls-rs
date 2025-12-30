// Generated macro for merge_base (module)
macro_rules! Depcrate_repositorymerge_base {
() => {
// Module: crate::repository
// Provides: {"merge_base"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "revision")] pub mod merge_base { # [doc = " The error returned by [Repository::merge_base()](crate::Repository::merge_base())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] OpenCache (# [from] crate :: repository :: commit_graph_if_enabled :: Error) , # [error (transparent)] FindMergeBase (# [from] gix_revision :: merge_base :: Error) , # [error ("Could not find a merge-base between commits {first} and {second}")] NotFound { first : gix_hash :: ObjectId , second : gix_hash :: ObjectId , } , } }
};
}
