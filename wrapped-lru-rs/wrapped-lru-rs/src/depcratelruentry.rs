// Generated macro for LruEntry (struct)
macro_rules! DepcrateLruEntry {
() => {
// Module: crate
// Provides: {"LruEntry"}
// Dependencies: {}
struct LruEntry < K , V > { key : mem :: MaybeUninit < K > , val : mem :: MaybeUninit < V > , prev : * mut LruEntry < K , V > , next : * mut LruEntry < K , V > , }
};
}
