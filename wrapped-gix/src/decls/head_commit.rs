macro_rules! deps {
    () => {
        Error!();
        Head!();
    };
}

macro_rules! head_commit {
    () => {
        deps!();
        # [doc = ""] pub mod head_commit { # [doc = " The error returned by [`Repository::head_commit`(…)](crate::Repository::head_commit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Head (# [from] crate :: reference :: find :: existing :: Error) , # [error (transparent)] PeelToCommit (# [from] crate :: head :: peel :: to_commit :: Error) , } }
    };
}

head_commit!();