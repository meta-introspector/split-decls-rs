// Generated macro for Error (enum)
macro_rules! Depcrate_store_impls_loose_findError {
() => {
// Module: crate::store_impls::loose::find
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned by [`Store::try_find()`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("decompression of loose object at '{path}' failed")] DecompressFile { source : zlib :: inflate :: Error , path : PathBuf , } , # [error ("file at '{path}' showed invalid size of inflated data, expected {expected}, got {actual}")] SizeMismatch { actual : u64 , expected : u64 , path : PathBuf } , # [error (transparent)] Decode (# [from] gix_object :: decode :: LooseHeaderDecodeError) , # [error ("Cannot store {size} in memory as it's not representable")] OutOfMemory { size : u64 } , # [error ("Could not {action} data at '{path}'")] Io { source : std :: io :: Error , action : & 'static str , path : PathBuf , } , }
};
}
