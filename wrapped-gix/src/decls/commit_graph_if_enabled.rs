macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! commit_graph_if_enabled {
    () => {
        deps!();
        # [doc = ""] pub mod commit_graph_if_enabled { # [doc = " The error returned by [Repository::commit_graph_if_enabled()](crate::Repository::commit_graph_if_enabled())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ConfigBoolean (# [from] crate :: config :: boolean :: Error) , # [error (transparent)] OpenCommitGraph (# [from] gix_commitgraph :: init :: Error) , } }
    };
}

commit_graph_if_enabled!();