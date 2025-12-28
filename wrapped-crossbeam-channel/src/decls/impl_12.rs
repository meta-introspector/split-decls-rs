macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T > RefUnwindSafe for Sender < T > { }
    };
}

impl_12!();