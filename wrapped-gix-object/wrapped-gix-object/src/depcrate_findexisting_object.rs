// Generated macro for existing_object (module)
macro_rules! Depcrate_findexisting_object {
() => {
// Module: crate::find
// Provides: {"existing_object"}
// Dependencies: {}
# [doc = ""] pub mod existing_object { use gix_hash :: ObjectId ; # [doc = " The error returned by the various [`find_*()`][crate::FindExt::find_commit()] trait methods."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (crate :: find :: Error) , # [error ("Could not decode object at {oid}")] Decode { oid : ObjectId , source : crate :: decode :: Error , } , # [error ("An object with id {oid} could not be found")] NotFound { oid : ObjectId } , # [error ("Expected object of kind {expected} but got {actual} at {oid}")] ObjectKind { oid : ObjectId , actual : crate :: Kind , expected : crate :: Kind , } , } }
};
}
