macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! merge_bases_many {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "revision")] pub mod merge_bases_many { # [doc = " The error returned by [Repository::merge_bases_many()](crate::Repository::merge_bases_many())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] OpenCache (# [from] crate :: repository :: commit_graph_if_enabled :: Error) , # [error (transparent)] MergeBase (# [from] gix_revision :: merge_base :: Error) , } }
    };
}

merge_bases_many!();