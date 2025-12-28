macro_rules! deps {
    () => {
        ConflictStyle!();
        Error!();
    };
}

macro_rules! blob_merge_options {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "merge")] pub mod blob_merge_options { # [doc = " The error returned by [Repository::blob_merge_options()](crate::Repository::blob_merge_options())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] DiffAlgorithm (# [from] crate :: config :: diff :: algorithm :: Error) , # [error (transparent)] ConflictStyle (# [from] crate :: config :: key :: GenericErrorWithValue) , } }
    };
}

blob_merge_options!()