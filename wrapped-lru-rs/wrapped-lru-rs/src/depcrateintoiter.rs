// Generated macro for IntoIter (struct)
macro_rules! DepcrateIntoIter {
() => {
// Module: crate
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator that moves out of a `LruCache`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`LruCache`][`LruCache`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: struct.LruCache.html#method.into_iter"] # [doc = " [`LruCache`]: struct.LruCache.html"] pub struct IntoIter < K , V > where K : Hash + Eq , { cache : LruCache < K , V > , }
};
}
