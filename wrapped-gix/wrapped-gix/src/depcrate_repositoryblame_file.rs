// Generated macro for blame_file (module)
macro_rules! Depcrate_repositoryblame_file {
() => {
// Module: crate::repository
// Provides: {"blame_file"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "blame")] pub mod blame_file { # [doc = " Options to be passed to [Repository::blame_file()](crate::Repository::blame_file())."] # [derive (Default , Debug , Clone)] pub struct Options { # [doc = " The algorithm to use for diffing. If `None`, `diff.algorithm` will be used."] pub diff_algorithm : Option < gix_diff :: blob :: Algorithm > , # [doc = " The ranges to blame in the file."] pub ranges : gix_blame :: BlameRanges , # [doc = " Don't consider commits before the given date."] pub since : Option < gix_date :: Time > , # [doc = " Determine if rename tracking should be performed, and how."] pub rewrites : Option < gix_diff :: Rewrites > , } # [doc = " The error returned by [Repository::blame_file()](crate::Repository::blame_file())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] CommitGraphIfEnabled (# [from] super :: commit_graph_if_enabled :: Error) , # [error (transparent)] DiffAlgorithm (# [from] crate :: config :: diff :: algorithm :: Error) , # [error (transparent)] DiffResourceCache (# [from] super :: diff_resource_cache :: Error) , # [error (transparent)] Blame (# [from] gix_blame :: Error) , } }
};
}
