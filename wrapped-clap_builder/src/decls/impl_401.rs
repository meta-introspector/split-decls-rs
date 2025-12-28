macro_rules! deps {
    () => {
        Escape!();
        Result!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl std :: fmt :: Display for Escape < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if self . 0 . contains (char :: is_whitespace) { std :: fmt :: Debug :: fmt (self . 0 , f) } else { self . 0 . fmt (f) } } }
    };
}

impl_401!()