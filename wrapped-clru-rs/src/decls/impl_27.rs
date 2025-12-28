macro_rules! deps {
    () => {
        CLruCache!();
        WeightScale!();
        FixedSizeList!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < K : Eq + Hash , V , W : WeightScale < K , V > > CLruCache < K , V , RandomState , W > { # [doc = " Creates a new LRU cache that holds at most `capacity` elements"] # [doc = " and uses the provided scale to retrieve value's weight."] pub fn with_scale (capacity : NonZeroUsize , scale : W) -> CLruCache < K , V , RandomState , W > { Self { lookup : HashMap :: with_hasher (RandomState :: default ()) , storage : FixedSizeList :: new (capacity . get ()) , scale , weight : 0 , } } }
    };
}

impl_27!();