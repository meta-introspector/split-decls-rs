macro_rules! deps {
    () => {
        ErrnoSentinel!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl ErrnoSentinel for * mut c_void { fn sentinel () -> Self { - 1isize as * mut c_void } }
    };
}

impl_13!()