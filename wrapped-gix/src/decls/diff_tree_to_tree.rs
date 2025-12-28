macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! diff_tree_to_tree {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "blob-diff")] pub mod diff_tree_to_tree { # [doc = " The error returned by [Repository::diff_tree_to_tree()](crate::Repository::diff_tree_to_tree())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] DiffOptions (# [from] crate :: diff :: options :: init :: Error) , # [error (transparent)] CreateResourceCache (# [from] super :: diff_resource_cache :: Error) , # [error (transparent)] TreeDiff (# [from] gix_diff :: tree_with_rewrites :: Error) , } }
    };
}

diff_tree_to_tree!();