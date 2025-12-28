macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArraySize > From < GenericArray < T , N > > for HybridArray < T , N > { # [inline (always)] fn from (value : GenericArray < T , N >) -> Self { value . into_ha0_4 () } }
    };
}

impl_12!()