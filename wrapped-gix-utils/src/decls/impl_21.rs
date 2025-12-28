macro_rules! deps {
    () => {
        ParseIntegerError!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl std :: fmt :: Display for ParseIntegerError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . desc () . fmt (f) } }
    };
}

impl_21!()