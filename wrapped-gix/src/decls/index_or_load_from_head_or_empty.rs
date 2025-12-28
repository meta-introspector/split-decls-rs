macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! index_or_load_from_head_or_empty {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "index")] pub mod index_or_load_from_head_or_empty { # [doc = " The error returned by [`Repository::index_or_load_from_head_or_empty()`](crate::Repository::index_or_load_from_head_or_empty())."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ReadHead (# [from] crate :: reference :: find :: existing :: Error) , # [error (transparent)] FindCommit (# [from] crate :: object :: find :: existing :: Error) , # [error (transparent)] PeelToTree (# [from] crate :: object :: peel :: to_kind :: Error) , # [error (transparent)] TreeId (# [from] gix_object :: decode :: Error) , # [error (transparent)] TraverseTree (# [from] crate :: repository :: index_from_tree :: Error) , # [error (transparent)] OpenIndex (# [from] crate :: worktree :: open_index :: Error) , } }
    };
}

index_or_load_from_head_or_empty!()