macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArrayLength_0_14 < T > > From < GenericArray_0_14 < T , N > > for GenericArray < T , N > { # [inline (always)] fn from (value : GenericArray_0_14 < T , N >) -> Self { GenericArray :: from_0_14 (value) } }
    };
}

impl_2!()