// Generated macro for Error (struct)
macro_rules! Depcrate_verifyError {
() => {
// Module: crate::verify
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`oid::verify()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] # [error ("Hash was {actual}, but should have been {expected}")] pub struct Error { pub actual : ObjectId , pub expected : ObjectId , }
};
}
