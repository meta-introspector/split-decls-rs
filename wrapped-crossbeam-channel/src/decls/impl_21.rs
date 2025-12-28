macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T > UnwindSafe for Receiver < T > { }
    };
}

impl_21!()