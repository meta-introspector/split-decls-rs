// Generated macro for checksum (module)
macro_rules! Depcrate_file_verifychecksum {
() => {
// Module: crate::file::verify
// Provides: {"checksum"}
// Dependencies: {}
# [doc = ""] pub mod checksum { # [doc = " The error used in [`super::File::verify_checksum()`]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("failed to hash commit graph file")] Hasher (# [from] gix_hash :: hasher :: Error) , # [error (transparent)] Verify (# [from] gix_hash :: verify :: Error) , } }
};
}
