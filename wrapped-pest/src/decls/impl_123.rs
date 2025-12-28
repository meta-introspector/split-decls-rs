macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl Hash for Position < '_ > { fn hash < H : Hasher > (& self , state : & mut H) { (self . input as * const str) . hash (state) ; self . pos . hash (state) ; } }
    };
}

impl_123!()