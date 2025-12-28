macro_rules! deps {
    () => {
        FilterOp!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Display for FilterOp { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
    };
}

impl_19!()