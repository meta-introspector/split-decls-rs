macro_rules! deps {
    () => {
        GenericParam!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl std :: fmt :: Debug for GenericParam < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("GenericParam") . field (& self . name ()) . finish () } }
    };
}

impl_91!();