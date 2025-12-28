macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Hash for Ident { fn hash < H : Hasher > (& self , state : & mut H) { self . name . hash (state) ; } }
    };
}

impl_8!();