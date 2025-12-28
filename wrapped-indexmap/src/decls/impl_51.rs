macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < K , V > IndexMap < K , V > { # [doc = " Create a new map. (Does not allocate.)"] # [inline] pub fn new () -> Self { Self :: with_capacity (0) } # [doc = " Create a new map with capacity for `n` key-value pairs. (Does not"] # [doc = " allocate if `n` is zero.)"] # [doc = ""] # [doc = " Computes in **O(n)** time."] # [inline] pub fn with_capacity (n : usize) -> Self { Self :: with_capacity_and_hasher (n , < _ > :: default ()) } }
    };
}

impl_51!();