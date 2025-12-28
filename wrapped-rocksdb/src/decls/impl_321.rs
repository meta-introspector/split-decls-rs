macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl core :: fmt :: Debug for PropertyName { # [inline] fn fmt (& self , fmtr : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . as_str () . fmt (fmtr) } }
    };
}

impl_321!();