macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < T : fmt :: Binary > fmt :: Binary for Wrapping < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_432!()