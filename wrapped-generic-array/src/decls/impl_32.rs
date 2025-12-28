macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T , N : ArrayLength > Borrow < [T] > for GenericArray < T , N > { # [inline (always)] fn borrow (& self) -> & [T] { self . as_slice () } }
    };
}

impl_32!()