macro_rules! deps {
    () => {
        Hash128!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl std :: hash :: Hash for Hash128 { fn hash < H : std :: hash :: Hasher > (& self , h : & mut H) { h . write_u64 (self . truncate () . as_u64 ()) ; } }
    };
}

impl_7!()