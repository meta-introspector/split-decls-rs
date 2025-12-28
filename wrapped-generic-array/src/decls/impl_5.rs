macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArrayLength_0_14 < T > > AsMut < GenericArray_0_14 < T , N > > for GenericArray < T , N > { # [inline (always)] fn as_mut (& mut self) -> & mut GenericArray_0_14 < T , N > { self . as_0_14_mut () } }
    };
}

impl_5!()