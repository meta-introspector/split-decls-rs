macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl Hash for Span < '_ > { fn hash < H : Hasher > (& self , state : & mut H) { (self . input as * const str) . hash (state) ; self . start . hash (state) ; self . end . hash (state) ; } }
    };
}

impl_156!()