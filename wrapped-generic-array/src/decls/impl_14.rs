macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArraySize > AsMut < HybridArray < T , N > > for GenericArray < T , N > { # [inline (always)] fn as_mut (& mut self) -> & mut HybridArray < T , N > { self . as_ha0_4_mut () } }
    };
}

impl_14!();