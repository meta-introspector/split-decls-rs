// Generated macro for new_commit_as (module)
macro_rules! Depcrate_repositorynew_commit_as {
() => {
// Module: crate::repository
// Provides: {"new_commit_as"}
// Dependencies: {}
# [doc = ""] mod new_commit_as { # [doc = " The error returned by [`new_commit_as(…)`](crate::Repository::new_commit_as())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] WriteObject (# [from] crate :: object :: write :: Error) , # [error (transparent)] FindCommit (# [from] crate :: object :: find :: existing :: Error) , } }
};
}
