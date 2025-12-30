// Generated macro for CacheKvStore (trait)
macro_rules! Depcrate_incremental_cacheCacheKvStore {
() => {
// Module: crate::incremental_cache
// Provides: {"CacheKvStore"}
// Dependencies: {}
# [doc = " Backing storage for an incremental compilation cache, when enabled."] pub trait CacheKvStore { # [doc = " Given a cache key hash, retrieves the associated opaque serialized data."] fn get (& self , key : & [u8]) -> Option < Cow < [u8] > > ; # [doc = " Given a new cache key and a serialized blob obtained from `serialize_compiled`, stores it"] # [doc = " in the cache store."] fn insert (& mut self , key : & [u8] , val : Vec < u8 >) ; }
};
}
