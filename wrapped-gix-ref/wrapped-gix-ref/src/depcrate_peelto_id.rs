// Generated macro for to_id (module)
macro_rules! Depcrate_peelto_id {
() => {
// Module: crate::peel
// Provides: {"to_id"}
// Dependencies: {}
# [doc = ""] pub mod to_id { use gix_object :: bstr :: BString ; # [doc = " The error returned by [`crate::file::ReferenceExt::peel_to_id()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FollowToObject (# [from] super :: to_object :: Error) , # [error ("An error occurred when trying to resolve an object a reference points to")] Find (# [from] gix_object :: find :: Error) , # [error ("Object {oid} as referred to by {name:?} could not be found")] NotFound { oid : gix_hash :: ObjectId , name : BString } , } }
};
}
