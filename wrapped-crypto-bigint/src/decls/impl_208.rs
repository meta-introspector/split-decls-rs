macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < T > fmt :: Display for NonZero < T > where T : fmt :: Display + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_208!();