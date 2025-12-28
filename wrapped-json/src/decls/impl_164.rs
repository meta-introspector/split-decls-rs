macro_rules! deps {
    () => {
        CompactFormatter!();
        Formatter!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl Formatter for CompactFormatter { }
    };
}

impl_164!();