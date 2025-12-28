macro_rules! deps {
    () => {
        HashMap!();
        HashMapExt!();
        RandomState!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < K , V > HashMapExt for std :: collections :: HashMap < K , V , RandomState > { # [inline (always)] fn new () -> Self { Self :: with_hasher (RandomState :: default ()) } # [inline (always)] fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , RandomState :: default ()) } }
    };
}

impl_37!();