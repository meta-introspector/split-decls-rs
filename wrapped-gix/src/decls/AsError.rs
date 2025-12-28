macro_rules! deps {
    () => {
        Note!();
        Name!();
        Error!();
    };
}

macro_rules! AsError {
    () => {
        deps!();
        # [doc = " The error returned by [`Remote::save_as_to()`]."] # [doc = ""] # [doc = " Note that this type should rather be in the `as` module, but cannot be as it's part of the Rust syntax."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum AsError { # [error (transparent)] Save (# [from] Error) , # [error (transparent)] Name (# [from] crate :: remote :: name :: Error) , }
    };
}

AsError!()