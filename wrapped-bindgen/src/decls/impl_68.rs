macro_rules! impl_68 {
    () => {
        impl std :: fmt :: Debug for Attribute { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("Attribute") . field (& self . name ()) . finish () } }
    };
}

impl_68!();