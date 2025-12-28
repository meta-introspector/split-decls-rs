macro_rules! deps {
    () => {
        HashSetExt!();
        HashSet!();
        FixedState!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < T > HashSetExt for std :: collections :: HashSet < T , FixedState > { # [inline (always)] fn new () -> Self { Self :: with_hasher (FixedState :: default ()) } # [inline (always)] fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , FixedState :: default ()) } }
    };
}

impl_41!()