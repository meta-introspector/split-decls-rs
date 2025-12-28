macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArraySize > AsArrayMut < T > for GenericArray < T , N > { # [inline (always)] fn as_array_mut (& mut self) -> & mut HybridArray < T , N > { self . as_ha0_4_mut () } }
    };
}

impl_10!();