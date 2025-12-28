macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl std :: fmt :: Display for TypeName { fn fmt (& self , fmt : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (fmt , "{}.{}" , self . 0 , self . 1) } }
    };
}

impl_230!()