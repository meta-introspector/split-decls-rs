macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < T , N : ArrayLength > AsMutSlice for GenericArray < T , N > { # [inline (always)] fn as_mut_slice (& mut self) -> & mut [T] { self . as_mut () } }
    };
}

impl_96!();