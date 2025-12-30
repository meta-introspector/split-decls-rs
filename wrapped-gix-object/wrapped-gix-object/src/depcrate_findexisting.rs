// Generated macro for existing (module)
macro_rules! Depcrate_findexisting {
() => {
// Module: crate::find
// Provides: {"existing"}
// Dependencies: {}
# [doc = ""] pub mod existing { use gix_hash :: ObjectId ; # [doc = " The error returned by the [`find(…)`][crate::FindExt::find()] trait methods."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (crate :: find :: Error) , # [error ("An object with id {} could not be found" , . oid)] NotFound { oid : ObjectId } , } }
};
}
