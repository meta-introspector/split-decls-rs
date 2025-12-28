macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        unsafe impl < T : Send , N : ArrayLength > Send for GenericArray < T , N > { }
    };
}

impl_179!();