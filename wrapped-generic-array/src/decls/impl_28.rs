macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T : Eq , N : ArrayLength > Eq for GenericArray < T , N > { }
    };
}

impl_28!();