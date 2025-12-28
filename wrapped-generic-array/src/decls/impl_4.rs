macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArrayLength_0_14 < T > > AsRef < GenericArray_0_14 < T , N > > for GenericArray < T , N > { # [inline (always)] fn as_ref (& self) -> & GenericArray_0_14 < T , N > { self . as_0_14 () } }
    };
}

impl_4!();