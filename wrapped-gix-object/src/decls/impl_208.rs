macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl fmt :: Display for Kind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (std :: str :: from_utf8 (self . as_bytes ()) . expect ("Converting Kind name to utf8")) } }
    };
}

impl_208!();