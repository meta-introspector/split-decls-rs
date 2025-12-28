macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl < T : fmt :: Display > fmt :: Display for Wrapping < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_431!()