macro_rules! deps {
    () => {
        DisplayBuffer!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl core :: fmt :: Display for DisplayBuffer { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (self . as_str ()) } }
    };
}

impl_21!()