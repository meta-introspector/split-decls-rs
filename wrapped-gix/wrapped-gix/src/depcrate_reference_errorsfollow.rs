// Generated macro for follow (module)
macro_rules! Depcrate_reference_errorsfollow {
() => {
// Module: crate::reference::errors
// Provides: {"follow"}
// Dependencies: {}
# [doc = ""] pub mod follow { # [doc = ""] pub mod to_object { # [doc = " The error returned by [`Reference::follow_to_object(…)`](crate::Reference::follow_to_object())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FollowToObject (# [from] gix_ref :: peel :: to_object :: Error) , # [error (transparent)] PackedRefsOpen (# [from] gix_ref :: packed :: buffer :: open :: Error) , } } }
};
}
