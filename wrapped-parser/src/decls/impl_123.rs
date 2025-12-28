macro_rules! deps {
    () => {
        Positioned!();
        Result!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < T : fmt :: Display > fmt :: Display for Positioned < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . node . fmt (f) } }
    };
}

impl_123!()