// Generated macro for CLruCacheIter (struct)
macro_rules! DepcrateCLruCacheIter {
() => {
// Module: crate
// Provides: {"CLruCacheIter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a `CLruCache`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`CLruCache`][`CLruCache`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`iter`]: struct.CLruCache.html#method.iter"] # [doc = " [`CLruCache`]: struct.CLruCache.html"] # [derive (Clone , Debug)] pub struct CLruCacheIter < 'a , K , V > { iter : FixedSizeListIter < 'a , CLruNode < K , V > > , }
};
}
