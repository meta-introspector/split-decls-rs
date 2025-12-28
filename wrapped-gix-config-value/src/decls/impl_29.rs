macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Display for Integer { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . value) ? ; if let Some (suffix) = self . suffix { write ! (f , "{suffix}") } else { Ok (()) } } }
    };
}

impl_29!();