macro_rules! deps {
    () => {
        Errors!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl core :: fmt :: Display for Errors { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { core :: fmt :: Debug :: fmt (self , f) } }
    };
}

impl_84!();