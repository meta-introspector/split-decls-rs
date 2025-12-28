macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T > RefUnwindSafe for Receiver < T > { }
    };
}

impl_22!()