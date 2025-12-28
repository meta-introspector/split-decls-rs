macro_rules! deps {
    () => {
        LexError!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl Error for LexError { }
    };
}

impl_202!();