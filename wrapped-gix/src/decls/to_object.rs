macro_rules! deps {
    () => {
        Branch!();
        Error!();
    };
}

macro_rules! to_object {
    () => {
        deps!();
        # [doc = ""] pub mod to_object { # [doc = " The error returned by [`Head::peel_to_object()`](super::Head::peel_to_object())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Peel (# [from] super :: Error) , # [error ("Branch '{name}' does not have any commits")] Unborn { name : gix_ref :: FullName } , } }
    };
}

to_object!()