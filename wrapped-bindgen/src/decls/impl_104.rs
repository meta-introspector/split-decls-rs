macro_rules! impl_104 {
    () => {
        impl std :: fmt :: Debug for TypeDef { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "TypeDef({})" , self . type_name ()) } }
    };
}

impl_104!();