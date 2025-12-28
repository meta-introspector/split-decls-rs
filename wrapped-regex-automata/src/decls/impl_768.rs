macro_rules! deps {
    () => {
        SmallIndexError!();
    };
}

macro_rules! impl_768 {
    () => {
        deps!();
        impl SmallIndexError { # [doc = " Returns the value that could not be converted to a small index."] pub fn attempted (& self) -> u64 { self . attempted } }
    };
}

impl_768!()