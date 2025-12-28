macro_rules! deps {
    () => {
        Error!();
        Index!();
    };
}

macro_rules! index_id {
    () => {
        deps!();
        # [doc = ""] pub mod index_id { # [doc = " The error returned by [Submodule::index_id()](crate::Submodule::index_id())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] PathConfiguration (# [from] gix_submodule :: config :: path :: Error) , # [error (transparent)] Index (# [from] crate :: repository :: index_or_load_from_head :: Error) , } }
    };
}

index_id!()