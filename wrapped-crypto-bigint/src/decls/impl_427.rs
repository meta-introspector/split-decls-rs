macro_rules! deps {
    () => {
        Zero!();
        Wrapping!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl < T : Zero > Zero for Wrapping < T > { # [inline] fn zero () -> Self { Wrapping (T :: zero ()) } }
    };
}

impl_427!()