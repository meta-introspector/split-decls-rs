macro_rules! deps {
    () => {
        CompactFormatter!();
        Formatter!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl Formatter for CompactFormatter { }
    };
}

impl_216!();