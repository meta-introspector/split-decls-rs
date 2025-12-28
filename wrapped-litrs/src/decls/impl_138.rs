macro_rules! deps {
    () => {
        FloatType!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl fmt :: Display for FloatType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . suffix () . fmt (f) } }
    };
}

impl_138!()