macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! merge {
    () => {
        deps!();
        # [doc = ""] pub mod merge { # [doc = ""] pub mod pipeline_options { # [doc = " The error produced when obtaining options needed to fill in [gix_merge::blob::pipeline::Options]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] BigFileThreshold (# [from] crate :: config :: unsigned_integer :: Error) , } } # [doc = ""] pub mod drivers { # [doc = " The error produced when obtaining a list of [Drivers](gix_merge::blob::Driver)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ConfigBoolean (# [from] crate :: config :: boolean :: Error) , } } }
    };
}

merge!();