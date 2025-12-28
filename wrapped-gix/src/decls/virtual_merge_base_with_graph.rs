macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! virtual_merge_base_with_graph {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "merge")] pub mod virtual_merge_base_with_graph { # [doc = " The error returned by [Repository::virtual_merge_base_with_graph()](crate::Repository::virtual_merge_base_with_graph())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("No commit was provided as merge-base")] MissingCommit , # [error (transparent)] MergeResourceCache (# [from] super :: merge_resource_cache :: Error) , # [error (transparent)] DiffResourceCache (# [from] super :: diff_resource_cache :: Error) , # [error (transparent)] CommitMerge (# [from] gix_merge :: commit :: Error) , # [error (transparent)] FindCommit (# [from] crate :: object :: find :: existing :: with_conversion :: Error) , # [error (transparent)] DecodeCommit (# [from] gix_object :: decode :: Error) , } }
    };
}

virtual_merge_base_with_graph!()