// Generated macro for Error (enum)
macro_rules! Depcrate_describeError {
() => {
// Module: crate::describe
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by the [`describe()`][function::describe()] function."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The parents of commit {} could not be added to graph during traversal" , oid . to_hex ())] InsertParentsToGraph { # [source] err : crate :: graph :: insert_parents :: Error , oid : gix_hash :: ObjectId , } , # [error ("A commit could not be decoded during traversal")] Decode (# [from] gix_object :: decode :: Error) , }
};
}
