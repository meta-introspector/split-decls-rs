macro_rules! deps {
    () => {
        CLruNode!();
        CLruCache!();
    };
}

macro_rules! CLruCacheIterMut {
    () => {
        deps!();
        # [doc = " An iterator over mutables entries of a `CLruCache`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_mut`] method on [`CLruCache`][`CLruCache`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`iter_mut`]: struct.CLruCache.html#method.iter_mut"] # [doc = " [`CLruCache`]: struct.CLruCache.html"] pub struct CLruCacheIterMut < 'a , K , V > { iter : FixedSizeListIterMut < 'a , CLruNode < K , V > > , }
    };
}

CLruCacheIterMut!()