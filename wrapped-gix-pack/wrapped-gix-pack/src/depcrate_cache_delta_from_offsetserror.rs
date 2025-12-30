// Generated macro for Error (enum)
macro_rules! Depcrate_cache_delta_from_offsetsError {
() => {
// Module: crate::cache::delta::from_offsets
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned by [`Tree::from_offsets_in_pack()`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("{message}")] Io { source : io :: Error , message : & 'static str } , # [error (transparent)] Header (# [from] crate :: data :: header :: decode :: Error) , # [error ("Could find object with id {id} in this pack. Thin packs are not supported")] UnresolvedRefDelta { id : gix_hash :: ObjectId } , # [error (transparent)] Tree (# [from] crate :: cache :: delta :: Error) , # [error ("Interrupted")] Interrupted , }
};
}
