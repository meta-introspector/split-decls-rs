macro_rules! deps {
    () => {
        One!();
        Odd!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl < T > num_traits :: One for Odd < T > where T : One + Mul < T , Output = T > , { # [inline] fn one () -> Self { Self (T :: one ()) } fn is_one (& self) -> bool { self . 0 . is_one () . into () } }
    };
}

impl_236!();