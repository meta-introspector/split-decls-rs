// Generated macro for error (module)
macro_rules! Depcrate_file_verifyerror {
() => {
// Module: crate::file::verify
// Provides: {"error"}
// Dependencies: {}
mod error { # [doc = " The error returned by [File::verify_integrity()][super::File::verify_integrity()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not read index file to generate hash")] Io (# [from] gix_hash :: io :: Error) , # [error ("Index checksum mismatch")] Verify (# [from] gix_hash :: verify :: Error) , } }
};
}
