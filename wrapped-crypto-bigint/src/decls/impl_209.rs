macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < T > fmt :: Binary for NonZero < T > where T : fmt :: Binary + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Binary :: fmt (& self . 0 , f) } }
    };
}

impl_209!();