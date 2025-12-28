macro_rules! deps {
    () => {
        IndexSet!();
        IndexMap!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < T > IndexSet < T > { # [doc = " Create a new set. (Does not allocate.)"] pub fn new () -> Self { IndexSet { map : IndexMap :: new () , } } # [doc = " Create a new set with capacity for `n` elements."] # [doc = " (Does not allocate if `n` is zero.)"] # [doc = ""] # [doc = " Computes in **O(n)** time."] pub fn with_capacity (n : usize) -> Self { IndexSet { map : IndexMap :: with_capacity (n) , } } }
    };
}

impl_77!()