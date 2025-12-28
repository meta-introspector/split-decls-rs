macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T , N : ArrayLength > DerefMut for GenericArray < T , N > { # [inline (always)] fn deref_mut (& mut self) -> & mut [T] { GenericArray :: as_mut_slice (self) } }
    };
}

impl_36!()