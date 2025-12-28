macro_rules! impl_92 {
    () => {
        impl std :: fmt :: Debug for MethodDef { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("MethodDef") . field (& self . name ()) . finish () } }
    };
}

impl_92!();