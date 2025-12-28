macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T : Clone , N : ArrayLength > Clone for GenericArray < T , N > { # [inline] fn clone (& self) -> GenericArray < T , N > { self . map (Clone :: clone) } }
    };
}

impl_25!();