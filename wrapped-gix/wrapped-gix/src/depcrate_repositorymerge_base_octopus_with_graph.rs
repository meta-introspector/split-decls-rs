// Generated macro for merge_base_octopus_with_graph (module)
macro_rules! Depcrate_repositorymerge_base_octopus_with_graph {
() => {
// Module: crate::repository
// Provides: {"merge_base_octopus_with_graph"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "revision")] pub mod merge_base_octopus_with_graph { # [doc = " The error returned by [Repository::merge_base_octopus_with_graph()](crate::Repository::merge_base_octopus_with_graph())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("No commit was provided")] MissingCommit , # [error ("No merge base was found between the given commits")] NoMergeBase , # [error (transparent)] MergeBase (# [from] gix_revision :: merge_base :: Error) , } }
};
}
