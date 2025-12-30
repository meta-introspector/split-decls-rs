// Generated macro for error (module)
macro_rules! Depcrate_file_decodeerror {
() => {
// Module: crate::file::decode
// Provides: {"error"}
// Dependencies: {}
mod error { # [doc = " The value returned by [`crate::file::Index::from_bytes()`]"] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Sentinel value encountered while still processing chunks.")] EarlySentinelValue , # [error ("Sentinel value wasn't found, saw {:?}" , std :: str :: from_utf8 (actual . as_ref ()) . unwrap_or ("<non-ascii>"))] MissingSentinelValue { actual : crate :: Id } , # [error ("The chunk offset {offset} went past the file of length {file_length} - was it truncated?")] ChunkSizeOutOfBounds { offset : crate :: file :: Offset , file_length : u64 , } , # [error ("All chunk offsets must be incrementing.")] NonIncrementalChunkOffsets , # [error ("The chunk of kind {:?} was encountered more than once" , std :: str :: from_utf8 (kind . as_ref ()) . unwrap_or ("<non-ascii>"))] DuplicateChunk { kind : crate :: Id } , # [error ("The table of contents would be {expected} bytes, but got only {actual}")] TocTooSmall { actual : usize , expected : usize } , # [error ("Empty chunk indices are not allowed as the point of chunked files is to have chunks.")] Empty , } }
};
}
