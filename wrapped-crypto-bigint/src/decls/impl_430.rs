macro_rules! deps {
    () => {
        Wrapping!();
        One!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl < T : num_traits :: One + WrappingMul + PartialEq > num_traits :: One for Wrapping < T > { # [inline] fn one () -> Self { Wrapping (T :: one ()) } # [inline] fn is_one (& self) -> bool { self . 0 . is_one () } }
    };
}

impl_430!();