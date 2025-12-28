macro_rules! deps {
    () => {
        Branch!();
        Error!();
    };
}

macro_rules! into_id {
    () => {
        deps!();
        # [doc = ""] pub mod into_id { use crate :: object ; # [doc = " The error returned by [`Head::into_peeled_id()`](super::Head::into_peeled_id())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Peel (# [from] super :: Error) , # [error ("Branch '{name}' does not have any commits")] Unborn { name : gix_ref :: FullName } , # [error (transparent)] ObjectKind (# [from] object :: try_into :: Error) , } }
    };
}

into_id!()