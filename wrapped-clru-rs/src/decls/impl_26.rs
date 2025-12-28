macro_rules! deps {
    () => {
        CLruCache!();
        FixedSizeList!();
        ZeroWeightScale!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < K : Eq + Hash , V , S : BuildHasher > CLruCache < K , V , S > { # [doc = " Creates a new LRU cache that holds at most `capacity` elements"] # [doc = " and uses the provided hash builder to hash keys."] pub fn with_hasher (capacity : NonZeroUsize , hash_builder : S) -> CLruCache < K , V , S > { Self { lookup : HashMap :: with_hasher (hash_builder) , storage : FixedSizeList :: new (capacity . get ()) , scale : ZeroWeightScale , weight : 0 , } } }
    };
}

impl_26!()