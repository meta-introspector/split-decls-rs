// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < K : Eq + Hash , V > CLruCache < K , V > { # [doc = " Creates a new LRU cache that holds at most `capacity` elements."] pub fn new (capacity : NonZeroUsize) -> Self { Self { lookup : HashMap :: new () , storage : FixedSizeList :: new (capacity . get ()) , scale : ZeroWeightScale , weight : 0 , } } # [doc = " Creates a new LRU cache that holds at most `capacity` elements"] # [doc = " and pre-allocates memory in order to hold at least `reserve` elements"] # [doc = " without reallocating."] pub fn with_memory (capacity : NonZeroUsize , mut reserve : usize) -> Self { if reserve > capacity . get () { reserve = capacity . get () ; } Self { lookup : HashMap :: with_capacity (reserve) , storage : FixedSizeList :: with_memory (capacity . get () , reserve) , scale : ZeroWeightScale , weight : 0 , } } }
};
}
