macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T , N : ArrayLength > From < GenericArray < T , N > > for Box < [T] > { # [inline] fn from (value : GenericArray < T , N >) -> Self { Box :: new (value) . into_boxed_slice () } }
    };
}

impl_62!()