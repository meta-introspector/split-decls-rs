macro_rules! deps {
    () => {
        CacheKey!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl std :: hash :: Hash for CacheKey { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { if self . use_id { self . id . hash (state) ; self . is_link . hash (state) ; } else { self . location . hash (state) ; } } }
    };
}

impl_111!();