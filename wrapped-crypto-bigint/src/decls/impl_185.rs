macro_rules! deps {
    () => {
        NonZero!();
        One!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < T > One for NonZero < T > where T : One , { # [inline] fn one () -> Self { Self (T :: one ()) } }
    };
}

impl_185!();