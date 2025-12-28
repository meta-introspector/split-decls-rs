macro_rules! deps {
    () => {
        HashSetExt!();
        RandomState!();
        HashSet!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T > HashSetExt for std :: collections :: HashSet < T , RandomState > { # [inline (always)] fn new () -> Self { Self :: with_hasher (RandomState :: default ()) } # [inline (always)] fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , RandomState :: default ()) } }
    };
}

impl_40!()