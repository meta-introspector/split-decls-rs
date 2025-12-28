macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Reference < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Debug :: fmt (& self . inner , f) } }
    };
}

impl_279!();