macro_rules! deps {
    () => {
        ErrnoSentinel!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl ErrnoSentinel for isize { fn sentinel () -> Self { - 1 } }
    };
}

impl_10!()