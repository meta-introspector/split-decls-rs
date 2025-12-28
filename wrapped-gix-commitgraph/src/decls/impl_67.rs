macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl std :: fmt :: Display for Position { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_67!();