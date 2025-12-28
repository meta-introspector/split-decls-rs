macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T : Debug , N : ArrayLength > Debug for GenericArray < T , N > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { self . as_slice () . fmt (fmt) } }
    };
}

impl_31!()