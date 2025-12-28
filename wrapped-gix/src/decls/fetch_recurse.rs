macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! fetch_recurse {
    () => {
        deps!();
        # [doc = ""] pub mod fetch_recurse { # [doc = " The error returned by [Submodule::fetch_recurse()](crate::Submodule::fetch_recurse())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ModuleBoolean (# [from] gix_submodule :: config :: Error) , # [error (transparent)] ConfigurationFallback (# [from] crate :: config :: key :: GenericErrorWithValue) , } }
    };
}

fetch_recurse!();