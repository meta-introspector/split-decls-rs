// Generated macro for LruCache (struct)
macro_rules! DepcrateLruCache {
() => {
// Module: crate
// Provides: {"LruCache"}
// Dependencies: {}
# [doc = " An LRU Cache"] pub struct LruCache < K , V , S = DefaultHasher > { map : HashMap < KeyRef < K > , NonNull < LruEntry < K , V > > , S > , cap : NonZeroUsize , head : * mut LruEntry < K , V > , tail : * mut LruEntry < K , V > , }
};
}
