macro_rules! deps {
    () => {
        ErrnoSentinel!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl ErrnoSentinel for i32 { fn sentinel () -> Self { - 1 } }
    };
}

impl_11!();