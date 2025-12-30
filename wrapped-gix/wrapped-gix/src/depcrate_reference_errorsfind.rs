// Generated macro for find (module)
macro_rules! Depcrate_reference_errorsfind {
() => {
// Module: crate::reference::errors
// Provides: {"find"}
// Dependencies: {}
# [doc = ""] pub mod find { # [doc = ""] pub mod existing { use gix_ref :: PartialName ; # [doc = " The error returned by [`find_reference(…)`][crate::Repository::find_reference()], and others."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (# [from] crate :: reference :: find :: Error) , # [error ("The reference '{}' did not exist" , name . as_ref () . as_bstr ())] NotFound { name : PartialName } , } } # [doc = " The error returned by [`try_find_reference(…)`][crate::Repository::try_find_reference()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (# [from] gix_ref :: file :: find :: Error) , } }
};
}
