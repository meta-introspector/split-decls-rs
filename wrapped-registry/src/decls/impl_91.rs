macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Data { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { self . deref () . fmt (f) } }
    };
}

impl_91!();