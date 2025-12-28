macro_rules! deps {
    () => {
        Boolean!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Display for Boolean { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_7!()