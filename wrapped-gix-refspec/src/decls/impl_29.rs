macro_rules! deps {
    () => {
        Mapping!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl std :: hash :: Hash for Mapping < '_ , '_ > { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . lhs . hash (state) ; self . rhs . hash (state) ; } }
    };
}

impl_29!()