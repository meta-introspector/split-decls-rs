// Generated macro for new_commit (module)
macro_rules! Depcrate_repositorynew_commit {
() => {
// Module: crate::repository
// Provides: {"new_commit"}
// Dependencies: {}
# [doc = ""] mod new_commit { # [doc = " The error returned by [`new_commit(…)`](crate::Repository::new_commit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ParseTime (# [from] crate :: config :: time :: Error) , # [error ("Committer identity is not configured")] CommitterMissing , # [error ("Author identity is not configured")] AuthorMissing , # [error (transparent)] NewCommitAs (# [from] crate :: repository :: new_commit_as :: Error) , } }
};
}
