macro_rules! deps {
    () => {
        StringValueParser!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl StringValueParser { # [doc = " Implementation for [`ValueParser::string`]"] pub fn new () -> Self { Self { } } }
    };
}

impl_270!()