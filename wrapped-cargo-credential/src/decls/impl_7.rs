macro_rules! deps {
    () => {
        StringTypedError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl std :: fmt :: Display for StringTypedError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . message . fmt (f) } }
    };
}

impl_7!();