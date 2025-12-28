macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArraySize > AsRef < HybridArray < T , N > > for GenericArray < T , N > { # [inline (always)] fn as_ref (& self) -> & HybridArray < T , N > { self . as_ha0_4 () } }
    };
}

impl_13!();