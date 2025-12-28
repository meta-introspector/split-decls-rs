macro_rules! Error {
    () => {
        # [doc = " The error returned by [`crate::discover()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Discover (# [from] upwards :: Error) , # [error (transparent)] Open (# [from] crate :: open :: Error) , }
    };
}

Error!()