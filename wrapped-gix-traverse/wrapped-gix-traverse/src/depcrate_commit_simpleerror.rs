// Generated macro for Error (enum)
macro_rules! Depcrate_commit_simpleError {
() => {
// Module: crate::commit::simple
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error is part of the item returned by the [Ancestors](super::Simple) iterator."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (# [from] gix_object :: find :: existing_iter :: Error) , # [error (transparent)] ObjectDecode (# [from] gix_object :: decode :: Error) , }
};
}
