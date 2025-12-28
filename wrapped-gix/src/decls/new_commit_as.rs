macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! new_commit_as {
    () => {
        deps!();
        # [doc = ""] mod new_commit_as { # [doc = " The error returned by [`new_commit_as(…)`](crate::Repository::new_commit_as())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] WriteObject (# [from] crate :: object :: write :: Error) , # [error (transparent)] FindCommit (# [from] crate :: object :: find :: existing :: Error) , } }
    };
}

new_commit_as!();