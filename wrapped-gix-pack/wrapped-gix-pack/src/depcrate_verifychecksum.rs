// Generated macro for checksum (module)
macro_rules! Depcrate_verifychecksum {
() => {
// Module: crate::verify
// Provides: {"checksum"}
// Dependencies: {}
# [doc = ""] pub mod checksum { # [doc = " Returned by various methods to verify the checksum of a memory mapped file that might also exist on disk."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Interrupted by user")] Interrupted , # [error ("Failed to hash data")] Hasher (# [from] gix_hash :: hasher :: Error) , # [error (transparent)] Verify (# [from] gix_hash :: verify :: Error) , } }
};
}
