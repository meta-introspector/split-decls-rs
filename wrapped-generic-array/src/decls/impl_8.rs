macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArraySize > AssocArraySize for GenericArray < T , N > { type Size = N ; }
    };
}

impl_8!();