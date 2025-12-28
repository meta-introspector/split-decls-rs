macro_rules! deps {
    () => {
        OsStringValueParser!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl OsStringValueParser { # [doc = " Implementation for [`ValueParser::os_string`]"] pub fn new () -> Self { Self { } } }
    };
}

impl_274!();