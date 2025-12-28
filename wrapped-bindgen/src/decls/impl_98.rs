macro_rules! impl_98 {
    () => {
        impl std :: fmt :: Debug for ModuleRef { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("ModuleRef") . field (& self . name ()) . finish () } }
    };
}

impl_98!();