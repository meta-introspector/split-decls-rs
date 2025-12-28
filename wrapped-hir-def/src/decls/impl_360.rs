macro_rules! deps {
    () => {
        LocalDefMap!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl std :: hash :: Hash for LocalDefMap { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let LocalDefMap { extern_prelude } = self ; extern_prelude . len () . hash (state) ; for (name , (crate_root , extern_crate)) in extern_prelude { name . hash (state) ; crate_root . hash (state) ; extern_crate . hash (state) ; } } }
    };
}

impl_360!();