// Generated macro for error (module)
macro_rules! Depcrate_decode_headererror {
() => {
// Module: crate::decode::header
// Provides: {"error"}
// Dependencies: {}
mod error { # [doc = " The error produced when failing to decode an index header."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("{0}")] Corrupt (& 'static str) , # [error ("Index version {0} is not supported")] UnsupportedVersion (u32) , } }
};
}
