// Generated macro for upstream_branch_and_remote_name_for_tracking_branch (module)
macro_rules! Depcrate_repositoryupstream_branch_and_remote_name_for_tracking_branch {
() => {
// Module: crate::repository
// Provides: {"upstream_branch_and_remote_name_for_tracking_branch"}
// Dependencies: {}
# [doc = ""] pub mod upstream_branch_and_remote_name_for_tracking_branch { # [doc = " The error returned by [Repository::upstream_branch_and_remote_name_for_tracking_branch()](crate::Repository::upstream_branch_and_remote_for_tracking_branch())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The input branch '{}' needs to be a remote tracking branch" , full_name . as_bstr ())] BranchCategory { full_name : gix_ref :: FullName } , # [error (transparent)] FindRemote (# [from] crate :: remote :: find :: existing :: Error) , # [error ("Found ambiguous remotes without 1:1 mapping or more than one match: {}" , remotes . iter () . map (| r | r . as_bstr () . to_string ()) . collect ::< Vec < _ >> () . join (", "))] AmbiguousRemotes { remotes : Vec < crate :: remote :: Name < 'static > > } , # [error (transparent)] ValidateUpstreamBranch (# [from] gix_ref :: name :: Error) , } }
};
}
