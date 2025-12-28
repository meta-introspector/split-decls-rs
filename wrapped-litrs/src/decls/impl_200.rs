macro_rules! deps {
    () => {
        IntegerType!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl fmt :: Display for IntegerType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . suffix () . fmt (f) } }
    };
}

impl_200!();