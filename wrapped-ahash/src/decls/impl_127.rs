macro_rules! deps {
    () => {
        HashMapExt!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < K , V , S > HashMapExt for std :: collections :: HashMap < K , V , S > where S : BuildHasher + Default , { fn new () -> Self { std :: collections :: HashMap :: with_hasher (S :: default ()) } fn with_capacity (capacity : usize) -> Self { std :: collections :: HashMap :: with_capacity_and_hasher (capacity , S :: default ()) } }
    };
}

impl_127!();