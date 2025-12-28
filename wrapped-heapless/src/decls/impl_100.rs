macro_rules! deps {
    () => {
        CoreMap!();
        IndexMap!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > IndexMap < K , V , BuildHasherDefault < S > , N > { # [doc = " Creates an empty `IndexMap`."] pub const fn new () -> Self { const { assert ! (N > 1) ; assert ! (N . is_power_of_two ()) ; } Self { build_hasher : BuildHasherDefault :: new () , core : CoreMap :: new () , } } }
    };
}

impl_100!();