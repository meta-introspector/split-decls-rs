macro_rules! deps {
    () => {
        HirId!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fmt :: Display for HirId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{self:?}") } }
    };
}

impl_13!()