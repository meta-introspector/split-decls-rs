macro_rules! deps {
    () => {
        GenBlockKind!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl fmt :: Display for GenBlockKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . modifier () . fmt (f) } }
    };
}

impl_97!();