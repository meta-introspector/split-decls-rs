macro_rules! deps {
    () => {
        ErrnoSentinel!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl ErrnoSentinel for i64 { fn sentinel () -> Self { - 1 } }
    };
}

impl_12!()