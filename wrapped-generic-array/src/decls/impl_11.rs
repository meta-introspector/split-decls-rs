macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArraySize > From < HybridArray < T , N > > for GenericArray < T , N > { # [inline (always)] fn from (value : HybridArray < T , N >) -> Self { GenericArray :: from_ha0_4 (value) } }
    };
}

impl_11!();