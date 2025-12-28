macro_rules! deps {
    () => {
        Configuration!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        # [allow (deprecated)] impl fmt :: Debug for Configuration { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . builder . fmt (f) } }
    };
}

impl_328!();