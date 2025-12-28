macro_rules! deps {
    () => {
        DisplayDebug!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Display for DisplayDebug < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_165!();