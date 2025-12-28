macro_rules! deps {
    () => {
        Result!();
        ErrorKind!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl std :: fmt :: Display for ErrorKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . as_str () . unwrap_or_default () . fmt (f) } }
    };
}

impl_405!();