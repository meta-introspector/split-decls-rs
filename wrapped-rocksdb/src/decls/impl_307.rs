macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl core :: fmt :: Display for PropName { # [inline] fn fmt (& self , fmtr : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . as_str () . fmt (fmtr) } }
    };
}

impl_307!()