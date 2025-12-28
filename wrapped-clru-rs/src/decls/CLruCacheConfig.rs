macro_rules! deps {
    () => {
        ZeroWeightScale!();
    };
}

macro_rules! CLruCacheConfig {
    () => {
        deps!();
        # [doc = " A configuration structure used to create an LRU cache."] pub struct CLruCacheConfig < K , V , S = RandomState , W = ZeroWeightScale > { pub (crate) capacity : NonZeroUsize , pub (crate) hash_builder : S , pub (crate) reserve : Option < usize > , pub (crate) scale : W , _marker : PhantomData < (K , V) > , }
    };
}

CLruCacheConfig!();