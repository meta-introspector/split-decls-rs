macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArraySize > AsArrayRef < T > for GenericArray < T , N > { # [inline (always)] fn as_array_ref (& self) -> & HybridArray < T , N > { self . as_ha0_4 () } }
    };
}

impl_9!();