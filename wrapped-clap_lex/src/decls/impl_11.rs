macro_rules! deps {
    () => {
        ArgCursor!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl ArgCursor { fn new () -> Self { Self { cursor : 0 } } }
    };
}

impl_11!();