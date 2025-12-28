macro_rules! deps {
    () => {
        Result!();
        BadScheme!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl fmt :: Display for BadScheme { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("URL scheme is not allowed") } }
    };
}

impl_31!()