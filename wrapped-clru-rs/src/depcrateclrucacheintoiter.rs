// Generated macro for CLruCacheIntoIter (struct)
macro_rules! DepcrateCLruCacheIntoIter {
() => {
// Module: crate
// Provides: {"CLruCacheIntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the elements of a `CLruCache`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`CLruCache`]"] # [doc = " (provided by the `IntoIterator` trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: struct.CLruCache.html#method.into_iter"] pub struct CLruCacheIntoIter < K , V , S , W : WeightScale < K , V > > { cache : CLruCache < K , V , S , W > , }
};
}
