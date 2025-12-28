macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < T > fmt :: Display for Odd < T > where T : fmt :: Display + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_248!();