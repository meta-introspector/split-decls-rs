// Generated macro for merge_base_octopus (module)
macro_rules! Depcrate_repositorymerge_base_octopus {
() => {
// Module: crate::repository
// Provides: {"merge_base_octopus"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "revision")] pub mod merge_base_octopus { # [doc = " The error returned by [Repository::merge_base_octopus()](crate::Repository::merge_base_octopus())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] OpenCache (# [from] crate :: repository :: commit_graph_if_enabled :: Error) , # [error (transparent)] MergeBaseOctopus (# [from] super :: merge_base_octopus_with_graph :: Error) , } }
};
}
