macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl core :: fmt :: Display for PropertyName { # [inline] fn fmt (& self , fmtr : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . as_str () . fmt (fmtr) } }
    };
}

impl_320!()