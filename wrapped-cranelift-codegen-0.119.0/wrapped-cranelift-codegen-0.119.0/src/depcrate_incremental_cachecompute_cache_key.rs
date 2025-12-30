// Generated macro for compute_cache_key (function)
macro_rules! Depcrate_incremental_cachecompute_cache_key {
() => {
// Module: crate::incremental_cache
// Provides: {"compute_cache_key"}
// Dependencies: {}
# [doc = " Compute a cache key, and hash it on your behalf."] # [doc = ""] # [doc = " Since computing the `CacheKey` is a bit expensive, it should be done as least as possible."] pub fn compute_cache_key (isa : & dyn TargetIsa , func : & Function) -> CacheKeyHash { use core :: hash :: { Hash as _ , Hasher } ; use sha2 :: Digest as _ ; struct Sha256Hasher (sha2 :: Sha256) ; impl Hasher for Sha256Hasher { fn finish (& self) -> u64 { panic ! ("Sha256Hasher doesn't support finish!") ; } fn write (& mut self , bytes : & [u8]) { self . 0 . update (bytes) ; } } let cache_key = CacheKey :: new (isa , func) ; let mut hasher = Sha256Hasher (sha2 :: Sha256 :: new ()) ; cache_key . hash (& mut hasher) ; let hash : [u8 ; 32] = hasher . 0 . finalize () . into () ; CacheKeyHash (hash) }
};
}
