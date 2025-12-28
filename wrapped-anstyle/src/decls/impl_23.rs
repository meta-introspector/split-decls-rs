macro_rules! deps {
    () => {
        NullFormatter!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl core :: fmt :: Display for NullFormatter { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (self . 0) } }
    };
}

impl_23!()