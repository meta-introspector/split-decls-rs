macro_rules! deps {
    () => {
        Wrapping!();
        Zero!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl < T : num_traits :: Zero + WrappingAdd > num_traits :: Zero for Wrapping < T > { # [inline] fn zero () -> Self { Wrapping (T :: zero ()) } # [inline] fn is_zero (& self) -> bool { self . 0 . is_zero () } }
    };
}

impl_429!();