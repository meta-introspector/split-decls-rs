macro_rules! deps {
    () => {
        Limit!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl fmt :: Display for Limit { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_427!();