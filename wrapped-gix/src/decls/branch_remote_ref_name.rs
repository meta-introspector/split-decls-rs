macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! branch_remote_ref_name {
    () => {
        deps!();
        # [doc = ""] pub mod branch_remote_ref_name { # [doc = " The error returned by [Repository::branch_remote_ref_name()](crate::Repository::branch_remote_ref_name())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The configured name of the remote ref to merge wasn't valid")] ValidateFetchRemoteRefName (# [from] gix_validate :: reference :: name :: Error) , # [error (transparent)] PushDefault (# [from] crate :: config :: key :: GenericErrorWithValue) , # [error (transparent)] FindPushRemote (# [from] crate :: remote :: find :: existing :: Error) , } }
    };
}

branch_remote_ref_name!();