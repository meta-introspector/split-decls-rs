macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! head_tree_id {
    () => {
        deps!();
        # [doc = ""] pub mod head_tree_id { # [doc = " The error returned by [`Repository::head_tree_id`(…)](crate::Repository::head_tree_id())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] HeadCommit (# [from] crate :: reference :: head_commit :: Error) , # [error (transparent)] DecodeCommit (# [from] gix_object :: decode :: Error) , } }
    };
}

head_tree_id!();