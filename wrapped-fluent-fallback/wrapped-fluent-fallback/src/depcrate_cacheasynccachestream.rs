// Generated macro for AsyncCacheStream (struct)
macro_rules! Depcrate_cacheAsyncCacheStream {
() => {
// Module: crate::cache
// Provides: {"AsyncCacheStream"}
// Dependencies: {}
pub struct AsyncCacheStream < 'a , S , R > where S : Stream , { cache : & 'a AsyncCache < S , R > , curr : usize , }
};
}
