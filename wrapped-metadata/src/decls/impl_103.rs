macro_rules! deps {
    () => {
        MethodDef!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl std :: fmt :: Debug for MethodDef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("MethodDef") . field (& self . name ()) . finish () } }
    };
}

impl_103!();