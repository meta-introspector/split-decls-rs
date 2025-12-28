macro_rules! deps {
    () => {
        WeightScale!();
        ZeroWeightScale!();
        CLruNode!();
        FixedSizeList!();
    };
}

macro_rules! CLruCache {
    () => {
        deps!();
        # [doc = " A weighted LRU cache with mostly¹ constant time operations."] # [doc = ""] # [doc = " Each key-value pair in the cache can have a weight that is retrieved"] # [doc = " using the provided [`WeightScale`] implementation. The default scale is"] # [doc = " [`ZeroWeightScale`] and always return 0. The number of elements that"] # [doc = " can be stored in the cache is conditioned by the sum of [`CLruCache::len`]"] # [doc = " and [`CLruCache::weight`]:"] # [doc = ""] # [doc = " [`CLruCache::len`] + [`CLruCache::weight`] <= [`CLruCache::capacity`]"] # [doc = ""] # [doc = " Using the default [`ZeroWeightScale`] scale unlocks some useful APIs"] # [doc = " that can currently only be implemented for this scale. The most interesting"] # [doc = " ones are probably:"] # [doc = ""] # [doc = " * [`CLruCache::put`]"] # [doc = " * [`CLruCache::put_or_modify`]"] # [doc = " * [`CLruCache::try_put_or_modify`]"] # [doc = ""] # [doc = " But more generally, using [`ZeroWeightScale`] unlocks all methods that return"] # [doc = " a mutable reference to the value of an element."] # [doc = " This is because modifying the value of an element can lead to a modification"] # [doc = " of its weight and therefore would put the cache into an incoherent state."] # [doc = " For the same reason, it is a logic error for a value to change weight while"] # [doc = " being stored in the cache."] # [doc = ""] # [doc = " The cache requires the keys to be clonable because it will store 2 instances"] # [doc = " of each key in different internal data structures. If cloning a key can be"] # [doc = " expensive, you might want to consider using an `Rc` or an `Arc`."] # [doc = ""] # [doc = " Note 1: See [`CLruCache::put_with_weight`]"] # [derive (Debug)] pub struct CLruCache < K , V , S = RandomState , W : WeightScale < K , V > = ZeroWeightScale > { lookup : HashMap < K , usize , S > , storage : FixedSizeList < CLruNode < K , V > > , scale : W , weight : usize , }
    };
}

CLruCache!();