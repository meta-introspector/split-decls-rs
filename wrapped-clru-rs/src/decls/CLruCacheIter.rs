macro_rules! deps {
    () => {
        CLruNode!();
        FixedSizeListIter!();
        CLruCache!();
    };
}

macro_rules! CLruCacheIter {
    () => {
        deps!();
        # [doc = " An iterator over the entries of a `CLruCache`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`CLruCache`][`CLruCache`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`iter`]: struct.CLruCache.html#method.iter"] # [doc = " [`CLruCache`]: struct.CLruCache.html"] # [derive (Clone , Debug)] pub struct CLruCacheIter < 'a , K , V > { iter : FixedSizeListIter < 'a , CLruNode < K , V > > , }
    };
}

CLruCacheIter!();