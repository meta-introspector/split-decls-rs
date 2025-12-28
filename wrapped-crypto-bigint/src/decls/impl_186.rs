macro_rules! deps {
    () => {
        One!();
        NonZero!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < T > num_traits :: One for NonZero < T > where T : One + Mul < T , Output = T > , { # [inline] fn one () -> Self { Self (T :: one ()) } fn is_one (& self) -> bool { self . 0 . is_one () . into () } }
    };
}

impl_186!();