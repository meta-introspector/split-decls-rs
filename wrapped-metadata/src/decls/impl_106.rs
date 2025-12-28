macro_rules! impl_106 {
    () => {
        impl std :: fmt :: Debug for MethodParam < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("MethodParam") . field (& self . name ()) . finish () } }
    };
}

impl_106!();