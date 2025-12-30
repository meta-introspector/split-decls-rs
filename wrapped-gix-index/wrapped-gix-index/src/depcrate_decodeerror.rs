// Generated macro for error (module)
macro_rules! Depcrate_decodeerror {
() => {
// Module: crate::decode
// Provides: {"error"}
// Dependencies: {}
mod error { use crate :: { decode , extension } ; # [doc = " The error returned by [`State::from_bytes()`][crate::State::from_bytes()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Header (# [from] decode :: header :: Error) , # [error ("Could not hash index data")] Hasher (# [from] gix_hash :: hasher :: Error) , # [error ("Could not parse entry at index {index}")] Entry { index : u32 } , # [error ("Mandatory extension wasn't implemented or malformed.")] Extension (# [from] extension :: decode :: Error) , # [error ("Index trailer should have been {expected} bytes long, but was {actual}")] UnexpectedTrailerLength { expected : usize , actual : usize } , # [error ("Shared index checksum mismatch")] Verify (# [from] gix_hash :: verify :: Error) , } }
};
}
