macro_rules! deps {
    () => {
        NonEmptyStringValueParser!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl NonEmptyStringValueParser { # [doc = " Parse non-empty string values"] pub fn new () -> Self { Self { } } }
    };
}

impl_312!()