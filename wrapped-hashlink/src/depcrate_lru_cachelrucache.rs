// Generated macro for LruCache (struct)
macro_rules! Depcrate_lru_cacheLruCache {
() => {
// Module: crate::lru_cache
// Provides: {"LruCache"}
// Dependencies: {}
pub struct LruCache < K , V , S = DefaultHashBuilder > { map : LinkedHashMap < K , V , S > , max_size : usize , }
};
}
