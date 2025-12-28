macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        unsafe impl < T : Send , N : ArrayLength > Send for GenericArray < T , N > { }
    };
}

impl_33!()