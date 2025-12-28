macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl std :: hash :: Hash for Row < '_ > { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . file . hash (state) ; self . pos . hash (state) ; } }
    };
}

impl_62!();