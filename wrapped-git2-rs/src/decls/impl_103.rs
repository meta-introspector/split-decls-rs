macro_rules! deps {
    () => {
        ReferenceType!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl fmt :: Display for ReferenceType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . str () . fmt (f) } }
    };
}

impl_103!()