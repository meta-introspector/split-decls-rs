macro_rules! deps {
    () => {
        Pattern!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl std :: fmt :: Display for Pattern { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . to_bstring () . fmt (f) } }
    };
}

impl_4!();