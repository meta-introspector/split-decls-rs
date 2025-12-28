macro_rules! deps {
    () => {
        TypeSpec!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl std :: fmt :: Debug for TypeSpec < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("TypeSpec") . field (& self . 0) . finish () } }
    };
}

impl_123!()