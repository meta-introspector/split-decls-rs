macro_rules! deps {
    () => {
        ModuleRef!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ModuleRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("ModuleRef") . field (& self . name ()) . finish () } }
    };
}

impl_111!();