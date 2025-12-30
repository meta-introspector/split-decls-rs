// Generated macro for branch_remote_ref_name (module)
macro_rules! Depcrate_repositorybranch_remote_ref_name {
() => {
// Module: crate::repository
// Provides: {"branch_remote_ref_name"}
// Dependencies: {}
# [doc = ""] pub mod branch_remote_ref_name { # [doc = " The error returned by [Repository::branch_remote_ref_name()](crate::Repository::branch_remote_ref_name())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The configured name of the remote ref to merge wasn't valid")] ValidateFetchRemoteRefName (# [from] gix_validate :: reference :: name :: Error) , # [error (transparent)] PushDefault (# [from] crate :: config :: key :: GenericErrorWithValue) , # [error (transparent)] FindPushRemote (# [from] crate :: remote :: find :: existing :: Error) , } }
};
}
