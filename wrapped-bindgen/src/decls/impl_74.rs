macro_rules! impl_74 {
    () => {
        impl std :: fmt :: Debug for Constant { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("Constant") . field (& self . value ()) . finish () } }
    };
}

impl_74!();