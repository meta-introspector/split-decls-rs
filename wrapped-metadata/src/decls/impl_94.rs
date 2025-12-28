macro_rules! deps {
    () => {
        ImplMap!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ImplMap < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("ImplMap") . field (& self . import_name ()) . finish () } }
    };
}

impl_94!();