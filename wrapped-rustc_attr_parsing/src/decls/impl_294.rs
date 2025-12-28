macro_rules! deps {
    () => {
        PathParser!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl Display for PathParser < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , pprust :: path_to_string (& self . 0)) } }
    };
}

impl_294!();