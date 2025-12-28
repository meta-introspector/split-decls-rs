macro_rules! impl_83 {
    () => {
        impl std :: fmt :: Debug for ImplMap { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("ImplMap") . field (& self . import_name ()) . finish () } }
    };
}

impl_83!();