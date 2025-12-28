macro_rules! deps {
    () => {
        ArrayLength!();
        IntoArrayLength!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < N > IntoArrayLength for N where N : ArrayLength , { type ArrayLength = Self ; }
    };
}

impl_16!()