macro_rules! impl_80 {
    () => {
        impl std :: fmt :: Debug for GenericParam { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("GenericParam") . field (& self . name ()) . finish () } }
    };
}

impl_80!()