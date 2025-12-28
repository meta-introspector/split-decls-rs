macro_rules! impl_110 {
    () => {
        impl std :: fmt :: Debug for TypeSpec { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("TypeSpec") . field (& self . 0) . finish () } }
    };
}

impl_110!();