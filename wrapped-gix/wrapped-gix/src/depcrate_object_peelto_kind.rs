// Generated macro for to_kind (module)
macro_rules! Depcrate_object_peelto_kind {
() => {
// Module: crate::object::peel
// Provides: {"to_kind"}
// Dependencies: {}
# [doc = ""] pub mod to_kind { mod error { use crate :: object ; # [doc = " The error returned by [`Object::peel_to_kind()`][crate::Object::peel_to_kind()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FindExistingObject (# [from] object :: find :: existing :: Error) , # [error ("Last encountered object {oid} was {actual} while trying to peel to {expected}")] NotFound { oid : gix_hash :: Prefix , actual : object :: Kind , expected : object :: Kind , } , } } pub use error :: Error ; }
};
}
