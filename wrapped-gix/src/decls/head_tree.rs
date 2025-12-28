macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! head_tree {
    () => {
        deps!();
        # [doc = ""] pub mod head_tree { # [doc = " The error returned by [`Repository::head_tree`(…)](crate::Repository::head_tree())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] HeadCommit (# [from] crate :: reference :: head_commit :: Error) , # [error (transparent)] CommitTree (# [from] crate :: object :: commit :: Error) , } }
    };
}

head_tree!();