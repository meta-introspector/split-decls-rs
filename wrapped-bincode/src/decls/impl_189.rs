macro_rules! deps {
    () => {
        Compat!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < T > core :: fmt :: Display for Compat < T > where T : core :: fmt :: Display , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_189!()