macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < Z : ZeroizeOnDrop , N : ArrayLength > ZeroizeOnDrop for GenericArray < Z , N > { }
    };
}

impl_84!();