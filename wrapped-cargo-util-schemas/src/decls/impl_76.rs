macro_rules! deps {
    () => {
        RustVersion!();
        Result!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl Display for RustVersion { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_76!();