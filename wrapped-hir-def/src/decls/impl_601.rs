macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! impl_601 {
    () => {
        deps!();
        impl < N : AstIdNode > Hash for ItemLoc < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . container . hash (state) ; self . id . hash (state) ; } }
    };
}

impl_601!()