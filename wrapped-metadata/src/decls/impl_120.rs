macro_rules! deps {
    () => {
        TypeRef!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl std :: fmt :: Debug for TypeRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "TypeRef({}.{})" , self . namespace () , self . name ()) } }
    };
}

impl_120!()