macro_rules! deps {
    () => {
        One!();
        Wrapping!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < T : One > One for Wrapping < T > { # [inline] fn one () -> Self { Wrapping (T :: one ()) } }
    };
}

impl_428!()