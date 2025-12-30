// Generated macro for Error (enum)
macro_rules! Depcrate_treeError {
() => {
// Module: crate::tree
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`tree()`](super::tree())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (# [from] gix_object :: find :: existing_iter :: Error) , # [error ("The delegate cancelled the operation")] Cancelled , # [error (transparent)] EntriesDecode (# [from] gix_object :: decode :: Error) , }
};
}
