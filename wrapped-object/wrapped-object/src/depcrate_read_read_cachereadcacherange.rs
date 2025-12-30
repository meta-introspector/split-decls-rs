// Generated macro for ReadCacheRange (struct)
macro_rules! Depcrate_read_read_cacheReadCacheRange {
() => {
// Module: crate::read::read_cache
// Provides: {"ReadCacheRange"}
// Dependencies: {}
# [doc = " An implementation of [`ReadRef`] for a range of data in a stream that"] # [doc = " implements `Read + Seek`."] # [doc = ""] # [doc = " Shares an underlying [`ReadCache`] with a lifetime of `'a`."] # [derive (Debug)] pub struct ReadCacheRange < 'a , R : ReadCacheOps > { r : & 'a ReadCache < R > , offset : u64 , size : u64 , }
};
}
