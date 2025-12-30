// Generated macro for inflate (module)
macro_rules! Depcrate_zlibinflate {
() => {
// Module: crate::zlib
// Provides: {"inflate"}
// Dependencies: {}
# [doc = " non-streaming interfaces for decompression"] pub mod inflate { # [doc = " The error returned by various [Inflate methods][super::Inflate]"] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not write all bytes when decompressing content")] WriteInflated (# [from] std :: io :: Error) , # [error ("Could not decode zip stream, status was '{0}'")] Inflate (# [from] super :: DecompressError) , # [error ("The zlib status indicated an error, status was '{0:?}'")] Status (super :: Status) , } }
};
}
