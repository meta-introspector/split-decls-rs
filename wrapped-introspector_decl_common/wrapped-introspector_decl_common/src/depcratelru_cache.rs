// Generated macro for LRU_CACHE (static)
macro_rules! DepcrateLRU_CACHE {
() => {
// Module: crate
// Provides: {"LRU_CACHE"}
// Dependencies: {}
pub static LRU_CACHE : Lazy < Mutex < LruCache < String , Vec < u8 > > > > = Lazy :: new (| | { let capacity = NonZeroUsize :: new (1024) . expect ("Cache capacity must be non-zero") ; Mutex :: new (LruCache :: new (capacity)) }) ;
};
}
