// Generated macro for Error (enum)
macro_rules! Depcrate_hasherError {
() => {
// Module: crate::hasher
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`Hasher::try_finalize()`](crate::Hasher::try_finalize())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Detected SHA-1 collision attack with digest {digest}")] CollisionAttack { digest : crate :: ObjectId } , }
};
}
