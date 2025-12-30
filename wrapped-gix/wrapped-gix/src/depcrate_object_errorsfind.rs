// Generated macro for find (module)
macro_rules! Depcrate_object_errorsfind {
() => {
// Module: crate::object::errors
// Provides: {"find"}
// Dependencies: {}
# [doc = ""] pub mod find { # [doc = " Indicate that an error occurred when trying to find an object."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct Error (# [from] pub gix_object :: find :: Error) ; # [doc = ""] pub mod existing { # [doc = " An object could not be found in the database, or an error occurred when trying to obtain it."] pub type Error = gix_object :: find :: existing :: Error ; # [doc = ""] pub mod with_conversion { # [doc = " The error returned by [Repository::find_commit()](crate::Repository::find_commit)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (# [from] crate :: object :: find :: existing :: Error) , # [error (transparent)] Convert (# [from] crate :: object :: try_into :: Error) , } } } }
};
}
