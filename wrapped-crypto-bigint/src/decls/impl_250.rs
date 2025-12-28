macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl < T > fmt :: Octal for Odd < T > where T : fmt :: Octal + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Octal :: fmt (& self . 0 , f) } }
    };
}

impl_250!();