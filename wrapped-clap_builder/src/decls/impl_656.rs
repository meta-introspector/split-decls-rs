macro_rules! deps {
    () => {
        Result!();
        Id!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Id { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Debug :: fmt (self . as_str () , f) } }
    };
}

impl_656!();