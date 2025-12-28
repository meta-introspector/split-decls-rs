macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Display for Position { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_53!();