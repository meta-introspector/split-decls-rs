macro_rules! deps {
    () => {
        ValueRange!();
        Result!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ValueRange { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{self}") } }
    };
}

impl_163!();