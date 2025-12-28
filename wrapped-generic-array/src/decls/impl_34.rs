macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T , N : ArrayLength > AsRef < [T] > for GenericArray < T , N > { # [inline (always)] fn as_ref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_34!();