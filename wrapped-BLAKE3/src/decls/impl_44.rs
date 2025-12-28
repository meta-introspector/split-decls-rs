macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl fmt :: Display for Hash { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let hex = self . to_hex () ; let hex : & str = hex . as_str () ; f . write_str (hex) } }
    };
}

impl_44!()