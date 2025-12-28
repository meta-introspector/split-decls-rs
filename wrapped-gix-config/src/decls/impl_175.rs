macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl Display for Section < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . header) ? ; for event in & self . events { event . fmt (f) ? ; } Ok (()) } }
    };
}

impl_175!()