macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T , N : ArrayLength > From < GenericArray < T , N > > for Vec < T > { # [inline] fn from (value : GenericArray < T , N >) -> Self { Box :: < [T] > :: from (value) . into () } }
    };
}

impl_63!();