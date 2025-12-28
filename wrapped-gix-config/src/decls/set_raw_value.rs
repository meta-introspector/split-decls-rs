macro_rules! deps {
    () => {
        Header!();
        Error!();
    };
}

macro_rules! set_raw_value {
    () => {
        deps!();
        # [doc = ""] pub mod set_raw_value { # [doc = " The error returned by [`File::set_raw_value(…)`][crate::File::set_raw_value()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Header (# [from] crate :: parse :: section :: header :: Error) , # [error (transparent)] ValueName (# [from] crate :: parse :: section :: value_name :: Error) , } }
    };
}

set_raw_value!();