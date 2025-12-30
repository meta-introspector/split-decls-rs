// Generated macro for error (module)
macro_rules! Depcrate_object_commiterror {
() => {
// Module: crate::object::commit
// Provides: {"error"}
// Dependencies: {}
mod error { use crate :: object ; # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FindExistingObject (# [from] object :: find :: existing :: Error) , # [error ("The commit could not be decoded fully or partially")] Decode (# [from] gix_object :: decode :: Error) , # [error ("The commit date could not be parsed")] ParseDate (# [from] gix_date :: parse :: Error) , # [error ("Expected object of type {}, but got {}" , . expected , . actual)] ObjectKind { expected : gix_object :: Kind , actual : gix_object :: Kind , } , } }
};
}
