macro_rules! impl_39 {
    () => {
        impl Hash for FnAbi { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { core :: mem :: discriminant (& Self :: Unknown) . hash (state) ; } }
    };
}

impl_39!()