macro_rules! deps {
    () => {
        Error!();
        Tree!();
    };
}

macro_rules! extensions {
    () => {
        deps!();
        # [doc = ""] pub mod extensions { use crate :: extension ; # [doc = " The error returned by [`State::verify_extensions()`][crate::State::verify_extensions()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Tree (# [from] extension :: tree :: verify :: Error) , } }
    };
}

extensions!();