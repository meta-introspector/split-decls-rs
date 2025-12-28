macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < T , N : ArrayLength > AsSlice for GenericArray < T , N > { type Element = T ; # [inline (always)] fn as_slice (& self) -> & [T] { self . as_ref () } }
    };
}

impl_95!()