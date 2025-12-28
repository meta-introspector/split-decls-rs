macro_rules! deps {
    () => {
        Platform!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Platform < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Debug :: fmt (& self . path () , f) } }
    };
}

impl_17!();