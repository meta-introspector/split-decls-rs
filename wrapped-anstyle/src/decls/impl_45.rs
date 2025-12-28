macro_rules! deps {
    () => {
        Reset!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl core :: fmt :: Display for Reset { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (RESET) } }
    };
}

impl_45!()