macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! merge_trees {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "merge")] pub mod merge_trees { # [doc = " The error returned by [Repository::merge_trees()](crate::Repository::merge_trees())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] MergeResourceCache (# [from] super :: merge_resource_cache :: Error) , # [error (transparent)] DiffResourceCache (# [from] super :: diff_resource_cache :: Error) , # [error (transparent)] TreeMerge (# [from] gix_merge :: tree :: Error) , # [error (transparent)] ValidationOptions (# [from] crate :: config :: boolean :: Error) , } }
    };
}

merge_trees!()