macro_rules! deps {
    () => {
        HashSetExt!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < K , S > HashSetExt for std :: collections :: HashSet < K , S > where S : BuildHasher + Default , { fn new () -> Self { std :: collections :: HashSet :: with_hasher (S :: default ()) } fn with_capacity (capacity : usize) -> Self { std :: collections :: HashSet :: with_capacity_and_hasher (capacity , S :: default ()) } }
    };
}

impl_128!()