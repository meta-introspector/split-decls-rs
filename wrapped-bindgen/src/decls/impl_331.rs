macro_rules! deps {
    () => {
        Warnings!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl std :: fmt :: Display for Warnings { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { for message in & self . 0 { write ! (f , "{message}") ? ; } Ok (()) } }
    };
}

impl_331!()