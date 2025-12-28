macro_rules! deps {
    () => {
        Prefix!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Display for Prefix { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let f : & mut dyn fmt :: Write = f ; self . 0 . write_prefix (f) } }
    };
}

impl_10!()