macro_rules! deps {
    () => {
        Str!();
        Result!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl std :: fmt :: Display for Str { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_204!()