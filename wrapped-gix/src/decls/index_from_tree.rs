macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! index_from_tree {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "index")] pub mod index_from_tree { # [doc = " The error returned by [Repository::index_from_tree()](crate::Repository::index_from_tree)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not create index from tree at {id}")] IndexFromTree { id : gix_hash :: ObjectId , source : gix_index :: init :: from_tree :: Error , } , # [error ("Couldn't obtain configuration for core.protect*")] BooleanConfig (# [from] crate :: config :: boolean :: Error) , } }
    };
}

index_from_tree!()