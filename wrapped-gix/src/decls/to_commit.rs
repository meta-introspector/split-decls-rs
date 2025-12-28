macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! to_commit {
    () => {
        deps!();
        # [doc = ""] pub mod to_commit { use crate :: object ; # [doc = " The error returned by [`Head::peel_to_commit()`](super::Head::peel_to_commit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] PeelToObject (# [from] super :: to_object :: Error) , # [error (transparent)] ObjectKind (# [from] object :: try_into :: Error) , } }
    };
}

to_commit!()