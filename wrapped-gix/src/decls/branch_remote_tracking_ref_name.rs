macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! branch_remote_tracking_ref_name {
    () => {
        deps!();
        # [doc = ""] pub mod branch_remote_tracking_ref_name { # [doc = " The error returned by [Repository::branch_remote_tracking_ref_name()](crate::Repository::branch_remote_tracking_ref_name())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The name of the tracking reference was invalid")] ValidateTrackingRef (# [from] gix_validate :: reference :: name :: Error) , # [error ("Could not get the remote reference to translate into the local tracking branch")] RemoteRef (# [from] super :: branch_remote_ref_name :: Error) , # [error ("Couldn't find remote to obtain fetch-specs for mapping to the tracking reference")] FindRemote (# [from] crate :: remote :: find :: existing :: Error) , } }
    };
}

branch_remote_tracking_ref_name!();