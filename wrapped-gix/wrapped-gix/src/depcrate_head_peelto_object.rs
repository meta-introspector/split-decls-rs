// Generated macro for to_object (module)
macro_rules! Depcrate_head_peelto_object {
() => {
// Module: crate::head::peel
// Provides: {"to_object"}
// Dependencies: {}
# [doc = ""] pub mod to_object { # [doc = " The error returned by [`Head::peel_to_object()`](super::Head::peel_to_object())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Peel (# [from] super :: Error) , # [error ("Branch '{name}' does not have any commits")] Unborn { name : gix_ref :: FullName } , } }
};
}
