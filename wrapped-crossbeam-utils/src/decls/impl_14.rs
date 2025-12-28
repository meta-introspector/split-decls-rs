macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T > RefUnwindSafe for AtomicCell < T > { }
    };
}

impl_14!();