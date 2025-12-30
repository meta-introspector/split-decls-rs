// Generated macro for shorten (module)
macro_rules! Depcrate_idshorten {
() => {
// Module: crate::id
// Provides: {"shorten"}
// Dependencies: {}
# [doc = ""] pub mod shorten { # [doc = " Returned by [`Id::prefix()`][super::Id::shorten()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] PackedObjectsCount (# [from] gix_odb :: store :: load_index :: Error) , # [error (transparent)] DisambiguatePrefix (# [from] gix_odb :: store :: prefix :: disambiguate :: Error) , # [error ("Id could not be shortened as the object with id {} could not be found" , . oid)] NotFound { oid : gix_hash :: ObjectId } , } }
};
}
