macro_rules! deps {
    () => {
        ExpandError!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl fmt :: Display for ExpandError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . 1 . fmt (f) } }
    };
}

impl_49!()