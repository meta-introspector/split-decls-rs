macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T , N : ArrayLength > AsMut < [T] > for GenericArray < T , N > { # [inline (always)] fn as_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
    };
}

impl_35!();