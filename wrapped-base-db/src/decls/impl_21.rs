macro_rules! deps {
    () => {
        CrateName!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl fmt :: Display for CrateName { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_21!()