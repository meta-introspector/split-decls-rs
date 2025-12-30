// Generated macro for CacheKey (struct)
macro_rules! Depcrate_blob_platformCacheKey {
() => {
// Module: crate::blob::platform
// Provides: {"CacheKey"}
// Dependencies: {}
# [doc = " A key to uniquely identify either a location in the worktree, or in the object database."] # [derive (Clone)] pub (crate) struct CacheKey { id : gix_hash :: ObjectId , location : BString , # [doc = " If `true`, this is an `id` based key, otherwise it's location based."] use_id : bool , # [doc = " Only relevant when `id` is not null, to further differentiate content and allow us to"] # [doc = " keep track of both links and blobs with the same content (rare, but possible)."] is_link : bool , }
};
}
