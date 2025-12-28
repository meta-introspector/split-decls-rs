macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < T , N : ArrayLength > Deref for GenericArray < T , N > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & [T] { GenericArray :: as_slice (self) } }
    };
}

impl_181!();