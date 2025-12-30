// Generated macro for to_commit (module)
macro_rules! Depcrate_head_peelto_commit {
() => {
// Module: crate::head::peel
// Provides: {"to_commit"}
// Dependencies: {}
# [doc = ""] pub mod to_commit { use crate :: object ; # [doc = " The error returned by [`Head::peel_to_commit()`](super::Head::peel_to_commit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] PeelToObject (# [from] super :: to_object :: Error) , # [error (transparent)] ObjectKind (# [from] object :: try_into :: Error) , } }
};
}
