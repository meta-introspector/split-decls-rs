macro_rules! deps {
    () => {
        Variance!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl fmt :: Display for Variance { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let description = match self { Variance :: Bivariant => "bivariant" , Variance :: Covariant => "covariant" , Variance :: Contravariant => "contravariant" , Variance :: Invariant => "invariant" , } ; f . pad (description) } }
    };
}

impl_367!();