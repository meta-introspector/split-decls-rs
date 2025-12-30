// Generated macro for peel (module)
macro_rules! Depcrate_reference_errorspeel {
() => {
// Module: crate::reference::errors
// Provides: {"peel"}
// Dependencies: {}
# [doc = ""] pub mod peel { # [doc = " The error returned by [`Reference::peel_to_id()`](crate::Reference::peel_to_id()) and"] # [doc = " [`Reference::into_fully_peeled_id()`](crate::Reference::into_fully_peeled_id())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ToId (# [from] gix_ref :: peel :: to_id :: Error) , # [error (transparent)] PackedRefsOpen (# [from] gix_ref :: packed :: buffer :: open :: Error) , } # [doc = ""] pub mod to_kind { # [doc = " The error returned by [`Reference::peel_to_kind(…)`](crate::Reference::peel_to_kind())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FollowToObject (# [from] gix_ref :: peel :: to_object :: Error) , # [error (transparent)] PackedRefsOpen (# [from] gix_ref :: packed :: buffer :: open :: Error) , # [error (transparent)] FindObject (# [from] crate :: object :: find :: existing :: Error) , # [error (transparent)] PeelObject (# [from] crate :: object :: peel :: to_kind :: Error) , } } }
};
}
