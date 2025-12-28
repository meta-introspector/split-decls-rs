macro_rules! deps {
    () => {
        ArgValueCompleter!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ArgValueCompleter { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (type_name :: < Self > ()) } }
    };
}

impl_103!()