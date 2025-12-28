macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl fmt :: Debug for Endian { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . as_str ()) } }
    };
}

impl_65!()