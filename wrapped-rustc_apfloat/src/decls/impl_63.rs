macro_rules! deps {
    () => {
        DoubleFloat!();
        FloatConvert!();
        Fallback!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < F : FloatConvert < Fallback < F > > > fmt :: Display for DoubleFloat < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& Fallback :: from (* self) , f) } }
    };
}

impl_63!();