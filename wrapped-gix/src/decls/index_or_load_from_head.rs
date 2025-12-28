macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! index_or_load_from_head {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "index")] pub mod index_or_load_from_head { # [doc = " The error returned by [`Repository::index_or_load_from_head()`](crate::Repository::index_or_load_from_head())."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] HeadCommit (# [from] crate :: reference :: head_commit :: Error) , # [error (transparent)] TreeId (# [from] gix_object :: decode :: Error) , # [error (transparent)] TraverseTree (# [from] crate :: repository :: index_from_tree :: Error) , # [error (transparent)] OpenIndex (# [from] crate :: worktree :: open_index :: Error) , } }
    };
}

index_or_load_from_head!()