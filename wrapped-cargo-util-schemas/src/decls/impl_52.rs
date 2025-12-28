macro_rules! deps {
    () => {
        TomlLockfileSourceId!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl std :: hash :: Hash for TomlLockfileSourceId { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . kind . hash (state) ; self . url . hash (state) ; } }
    };
}

impl_52!();