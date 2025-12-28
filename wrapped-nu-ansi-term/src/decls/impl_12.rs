macro_rules! deps {
    () => {
        Suffix!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl fmt :: Display for Suffix { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let f : & mut dyn fmt :: Write = f ; self . 0 . write_suffix (f) } }
    };
}

impl_12!()