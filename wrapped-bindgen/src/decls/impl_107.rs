macro_rules! impl_107 {
    () => {
        impl std :: fmt :: Debug for TypeRef { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "TypeRef({})" , self . type_name ()) } }
    };
}

impl_107!()