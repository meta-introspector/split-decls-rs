macro_rules! deps {
    () => {
        PathBufValueParser!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl PathBufValueParser { # [doc = " Implementation for [`ValueParser::path_buf`]"] pub fn new () -> Self { Self { } } }
    };
}

impl_278!();