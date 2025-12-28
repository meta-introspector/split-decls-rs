macro_rules! deps {
    () => {
        CapacityError!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T > fmt :: Display for CapacityError < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , CAPERROR) } }
    };
}

impl_109!();