macro_rules! deps {
    () => {
        Str!();
        Result!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Str { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Debug :: fmt (self . as_str () , f) } }
    };
}

impl_205!()