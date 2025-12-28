macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl std :: hash :: Hash for Interface { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . def . hash (state) ; } }
    };
}

impl_304!();