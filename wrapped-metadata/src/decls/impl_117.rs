macro_rules! deps {
    () => {
        TypeDef!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl std :: fmt :: Debug for TypeDef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "TypeDef({}.{})" , self . namespace () , self . name ()) } }
    };
}

impl_117!();