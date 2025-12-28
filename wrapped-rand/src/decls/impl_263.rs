macro_rules! deps {
    () => {
        OsError!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl core :: fmt :: Display for OsError { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_263!()