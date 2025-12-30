// Generated macro for try_into (module)
macro_rules! Depcrate_objecttry_into {
() => {
// Module: crate::object
// Provides: {"try_into"}
// Dependencies: {}
# [doc = ""] pub mod try_into { # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] # [error ("Object named {id} was supposed to be of kind {expected}, but was kind {actual}.")] pub struct Error { pub actual : gix_object :: Kind , pub expected : gix_object :: Kind , pub id : gix_hash :: ObjectId , } }
};
}
