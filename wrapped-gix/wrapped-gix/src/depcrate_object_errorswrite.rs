// Generated macro for write (module)
macro_rules! Depcrate_object_errorswrite {
() => {
// Module: crate::object::errors
// Provides: {"write"}
// Dependencies: {}
# [doc = ""] pub mod write { # [doc = " An error to indicate writing to the loose object store failed."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct Error (# [from] pub gix_object :: write :: Error) ; }
};
}
