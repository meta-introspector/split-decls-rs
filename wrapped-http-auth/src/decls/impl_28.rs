macro_rules! deps {
    () => {
        ParamValue!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ParamValue < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "\"{}\"" , self . escaped) } }
    };
}

impl_28!()