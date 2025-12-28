macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! tree_merge_options {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "merge")] pub mod tree_merge_options { # [doc = " The error returned by [Repository::tree_merge_options()](crate::Repository::tree_merge_options())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] BlobMergeOptions (# [from] super :: blob_merge_options :: Error) , # [error (transparent)] RewritesConfig (# [from] crate :: diff :: new_rewrites :: Error) , # [error (transparent)] CommandContext (# [from] crate :: config :: command_context :: Error) , } }
    };
}

tree_merge_options!()