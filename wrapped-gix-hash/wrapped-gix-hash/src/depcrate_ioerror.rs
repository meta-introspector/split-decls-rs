// Generated macro for Error (enum)
macro_rules! Depcrate_ioError {
() => {
// Module: crate::io
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error type for I/O operations that compute hashes."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] std :: io :: Error) , # [error ("Failed to hash data")] Hasher (# [from] hasher :: Error) , }
};
}
