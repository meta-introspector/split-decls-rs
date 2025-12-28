macro_rules! deps {
    () => {
        SkipAny!();
    };
}

macro_rules! impl_826 {
    () => {
        deps!();
        impl < I > SkipAny < I > { # [doc = " Creates a new `SkipAny` iterator."] pub (super) fn new (base : I , count : usize) -> Self { SkipAny { base , count } } }
    };
}

impl_826!();