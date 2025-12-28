macro_rules! deps {
    () => {
        Id!();
        Result!();
    };
}

macro_rules! impl_655 {
    () => {
        deps!();
        impl std :: fmt :: Display for Id { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_655!()