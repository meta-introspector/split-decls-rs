macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl CustomFormat { # [doc = " Create a new custom `format` suitable for use with the [`jiff`] crate."] pub const fn new (format : & 'static str) -> Self { Self (format) } }
    };
}

impl_3!()