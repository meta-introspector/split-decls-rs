macro_rules! deps {
    () => {
        One!();
        Odd!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < T > One for Odd < T > where T : One , { # [inline] fn one () -> Self { Self (T :: one ()) } }
    };
}

impl_235!()