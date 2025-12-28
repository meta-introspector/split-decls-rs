macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T > UnwindSafe for Sender < T > { }
    };
}

impl_11!();