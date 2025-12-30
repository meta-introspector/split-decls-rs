// Generated macro for Error (enum)
macro_rules! Depcrate_cache_delta_traverseError {
() => {
// Module: crate::cache::delta::traverse
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned by [`Tree::traverse()`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("{message}")] ZlibInflate { source : gix_features :: zlib :: inflate :: Error , message : & 'static str , } , # [error ("The resolver failed to obtain the pack entry bytes for the entry at {pack_offset}")] ResolveFailed { pack_offset : u64 } , # [error (transparent)] EntryType (# [from] crate :: data :: entry :: decode :: Error) , # [error ("One of the object inspectors failed")] Inspect (# [from] Box < dyn std :: error :: Error + Send + Sync >) , # [error ("Interrupted")] Interrupted , # [error ("The base at {base_pack_offset} was referred to by a ref-delta, but it was never added to the tree as if the pack was still thin.")] OutOfPackRefDelta { # [doc = " The base's offset which was from a resolved ref-delta that didn't actually get added to the tree"] base_pack_offset : crate :: data :: Offset , } , # [error ("Failed to spawn thread when switching to work-stealing mode")] SpawnThread (# [from] std :: io :: Error) , # [error (transparent)] Delta (# [from] crate :: data :: delta :: apply :: Error) , }
};
}
