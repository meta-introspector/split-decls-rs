macro_rules! deps {
    () => {
        FixedState!();
        HashMap!();
        HashMapExt!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < K , V > HashMapExt for std :: collections :: HashMap < K , V , FixedState > { # [inline (always)] fn new () -> Self { Self :: with_hasher (FixedState :: default ()) } # [inline (always)] fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , FixedState :: default ()) } }
    };
}

impl_38!()