macro_rules! deps {
    () => {
        CLruCache!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < K : Eq + Hash , V > CLruCache < K , V > { # [doc = " Creates a new LRU cache that holds at most `capacity` elements."] pub fn new (capacity : NonZeroUsize) -> Self { Self { lookup : HashMap :: new () , storage : FixedSizeList :: new (capacity . get ()) , scale : ZeroWeightScale , weight : 0 , } } # [doc = " Creates a new LRU cache that holds at most `capacity` elements"] # [doc = " and pre-allocates memory in order to hold at least `reserve` elements"] # [doc = " without reallocating."] pub fn with_memory (capacity : NonZeroUsize , mut reserve : usize) -> Self { if reserve > capacity . get () { reserve = capacity . get () ; } Self { lookup : HashMap :: with_capacity (reserve) , storage : FixedSizeList :: with_memory (capacity . get () , reserve) , scale : ZeroWeightScale , weight : 0 , } } }
    };
}

impl_5!()