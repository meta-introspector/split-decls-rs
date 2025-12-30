// Generated macro for error (module)
macro_rules! Depcrate_head_peelerror {
() => {
// Module: crate::head::peel
// Provides: {"error"}
// Dependencies: {}
mod error { use crate :: { object , reference } ; # [doc = " The error returned by [`Head::peel_to_id()`](super::Head::try_peel_to_id()) and"] # [doc = " [`Head::into_fully_peeled_id()`](super::Head::try_into_peeled_id())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FindExistingObject (# [from] object :: find :: existing :: Error) , # [error (transparent)] PeelReference (# [from] reference :: peel :: Error) , } }
};
}
