macro_rules! deps {
    () => {
        IntoArrayLength!();
        ArrayLength!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < N > IntoArrayLength for N where N : ArrayLength , { type ArrayLength = Self ; }
    };
}

impl_162!()