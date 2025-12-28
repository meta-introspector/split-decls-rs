macro_rules! deps {
    () => {
        RawIterRange!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        unsafe impl < T > Send for RawIterRange < T > { }
    };
}

impl_70!();