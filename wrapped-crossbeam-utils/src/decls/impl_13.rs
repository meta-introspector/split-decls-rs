macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T > UnwindSafe for AtomicCell < T > { }
    };
}

impl_13!()