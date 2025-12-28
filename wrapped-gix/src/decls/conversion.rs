macro_rules! deps {
    () => {
        Kind!();
        Error!();
    };
}

macro_rules! conversion {
    () => {
        deps!();
        # [doc = ""] pub mod conversion { # [doc = " The error returned by [`crate::object::try_to_()`][crate::Object::try_to_commit_ref()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Decode (# [from] gix_object :: decode :: Error) , # [error ("Expected object type {}, but got {}" , . expected , . actual)] UnexpectedType { expected : gix_object :: Kind , actual : gix_object :: Kind , } , } }
    };
}

conversion!()