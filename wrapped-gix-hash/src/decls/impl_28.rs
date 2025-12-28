macro_rules! deps {
    () => {
        Prefix!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl std :: fmt :: Display for Prefix { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . bytes . to_hex_with_len (self . hex_len) . fmt (f) } }
    };
}

impl_28!()