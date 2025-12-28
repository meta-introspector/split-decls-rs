macro_rules! deps {
    () => {
        LexError!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Error for LexError { }
    };
}

impl_32!()