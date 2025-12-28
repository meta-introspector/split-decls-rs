macro_rules! deps {
    () => {
        Scheme!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl std :: fmt :: Display for Scheme { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (self . as_str ()) } }
    };
}

impl_11!();