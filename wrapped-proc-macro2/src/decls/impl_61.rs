macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl Hash for Ident { fn hash < H : Hasher > (& self , hasher : & mut H) { self . to_string () . hash (hasher) ; } }
    };
}

impl_61!()