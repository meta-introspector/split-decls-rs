// Generated macro for Error (enum)
macro_rules! Depcrate_tag_writeError {
() => {
// Module: crate::tag::write
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An Error used in [`Tag::write_to()`][crate::WriteTo::write_to()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Tags must not start with a dash: '-'")] StartsWithDash , # [error ("The tag name was no valid reference name")] InvalidRefName (# [from] gix_validate :: tag :: name :: Error) , }
};
}
