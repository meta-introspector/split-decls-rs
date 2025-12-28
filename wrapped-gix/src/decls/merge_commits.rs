macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! merge_commits {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "merge")] pub mod merge_commits { # [doc = " The error returned by [Repository::merge_commits()](crate::Repository::merge_commits())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] OpenCommitGraph (# [from] super :: commit_graph_if_enabled :: Error) , # [error (transparent)] MergeResourceCache (# [from] super :: merge_resource_cache :: Error) , # [error (transparent)] DiffResourceCache (# [from] super :: diff_resource_cache :: Error) , # [error (transparent)] CommitMerge (# [from] gix_merge :: commit :: Error) , # [error (transparent)] ValidationOptions (# [from] crate :: config :: boolean :: Error) , } }
    };
}

merge_commits!();