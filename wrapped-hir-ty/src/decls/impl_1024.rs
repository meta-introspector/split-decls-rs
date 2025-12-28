macro_rules! deps {
    () => {
        FnAbi!();
    };
}

macro_rules! impl_1024 {
    () => {
        deps!();
        impl Hash for FnAbi { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { core :: mem :: discriminant (& Self :: Unknown) . hash (state) ; } }
    };
}

impl_1024!();