macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < T > fmt :: Binary for Odd < T > where T : fmt :: Binary + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Binary :: fmt (& self . 0 , f) } }
    };
}

impl_249!();