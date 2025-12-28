macro_rules! deps {
    () => {
        SalsaAttr!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl std :: fmt :: Debug for SalsaAttr { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (fmt , "{:?}" , self . name) } }
    };
}

impl_25!()