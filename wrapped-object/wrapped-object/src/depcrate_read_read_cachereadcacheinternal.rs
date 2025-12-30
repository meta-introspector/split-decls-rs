// Generated macro for ReadCacheInternal (struct)
macro_rules! Depcrate_read_read_cacheReadCacheInternal {
() => {
// Module: crate::read::read_cache
// Provides: {"ReadCacheInternal"}
// Dependencies: {}
# [derive (Debug)] struct ReadCacheInternal < R : ReadCacheOps > { read : R , bufs : Map < (u64 , u64) , Box < [u8] > > , strings : Map < (u64 , u8) , Box < [u8] > > , len : Option < u64 > , }
};
}
