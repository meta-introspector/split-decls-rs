macro_rules! deps {
    () => {
        AstPtr!();
        AstNode!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < N : AstNode > Hash for AstPtr < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . raw . hash (state) } }
    };
}

impl_180!()