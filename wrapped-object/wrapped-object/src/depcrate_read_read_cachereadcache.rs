// Generated macro for ReadCache (struct)
macro_rules! Depcrate_read_read_cacheReadCache {
() => {
// Module: crate::read::read_cache
// Provides: {"ReadCache"}
// Dependencies: {}
# [doc = " An implementation of [`ReadRef`] for data in a stream that implements"] # [doc = " `Read + Seek`."] # [doc = ""] # [doc = " Contains a cache of read-only blocks of data, allowing references to"] # [doc = " them to be returned. Entries in the cache are never removed."] # [doc = " Entries are keyed on the offset and size of the read."] # [doc = " Currently overlapping reads are considered separate reads."] # [doc = ""] # [doc = " This is primarily intended for environments where memory mapped files"] # [doc = " are not available or not suitable, such as WebAssembly."] # [doc = ""] # [doc = " Note that malformed files can cause the cache to grow much larger than"] # [doc = " the file size."] # [derive (Debug)] pub struct ReadCache < R : ReadCacheOps > { cache : RefCell < ReadCacheInternal < R > > , }
};
}
