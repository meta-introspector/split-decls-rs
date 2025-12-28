macro_rules! deps {
    () => {
        CLruCache!();
        WeightScale!();
    };
}

macro_rules! CLruCacheIntoIter {
    () => {
        deps!();
        # [doc = " An owning iterator over the elements of a `CLruCache`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`CLruCache`]"] # [doc = " (provided by the `IntoIterator` trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: struct.CLruCache.html#method.into_iter"] pub struct CLruCacheIntoIter < K , V , S , W : WeightScale < K , V > > { cache : CLruCache < K , V , S , W > , }
    };
}

CLruCacheIntoIter!()