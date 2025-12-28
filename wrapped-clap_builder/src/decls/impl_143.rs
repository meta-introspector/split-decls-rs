macro_rules! impl_143 {
    () => {
        impl std :: hash :: Hash for Inner { # [inline] fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . as_os_str () . hash (state) ; } }
    };
}

impl_143!()