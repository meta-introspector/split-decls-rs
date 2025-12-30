// Generated macro for Error (enum)
macro_rules! Depcrate_data_input_typesError {
() => {
// Module: crate::data::input::types
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned by [`BytesToEntriesIter::new_from_header()`][crate::data::input::BytesToEntriesIter::new_from_header()] and as part"] # [doc = " of `Item` of [`BytesToEntriesIter`][crate::data::input::BytesToEntriesIter]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("An IO operation failed while streaming an entry")] Io (# [from] gix_hash :: io :: Error) , # [error (transparent)] PackParse (# [from] crate :: data :: header :: decode :: Error) , # [error ("Failed to verify pack checksum in trailer")] Verify (# [from] gix_hash :: verify :: Error) , # [error ("pack is incomplete: it was decompressed into {actual} bytes but {expected} bytes where expected.")] IncompletePack { actual : u64 , expected : u64 } , # [error ("The object {object_id} could not be decoded or wasn't found")] NotFound { object_id : gix_hash :: ObjectId } , }
};
}
