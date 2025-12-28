macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArrayLength_0_14 < T > > From < GenericArray < T , N > > for GenericArray_0_14 < T , N > { # [inline (always)] fn from (value : GenericArray < T , N >) -> Self { value . into_0_14 () } }
    };
}

impl_3!()