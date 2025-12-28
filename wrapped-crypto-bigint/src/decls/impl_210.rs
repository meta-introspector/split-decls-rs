macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < T > fmt :: Octal for NonZero < T > where T : fmt :: Octal + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Octal :: fmt (& self . 0 , f) } }
    };
}

impl_210!();