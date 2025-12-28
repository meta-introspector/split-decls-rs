macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! single {
    () => {
        deps!();
        # [doc = ""] pub mod single { use crate :: bstr :: BString ; # [doc = " The error returned by [`crate::Repository::rev_parse_single()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Parse (# [from] super :: Error) , # [error ("revspec {spec:?} did not resolve to a single object")] RangedRev { spec : BString } , } }
    };
}

single!()