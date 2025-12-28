macro_rules! deps {
    () => {
        AssocItemLoc!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < N : AstIdNode > Hash for AssocItemLoc < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . container . hash (state) ; self . id . hash (state) ; } }
    };
}

impl_36!()