macro_rules! deps {
    () => {
        NothingPrint!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Debug for NothingPrint { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "?") } }
    };
}

impl_13!()