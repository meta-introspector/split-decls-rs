macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl std :: hash :: Hash for Kind { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { match self { Kind :: Label (s) => { 0 . hash (state) ; s . dyn_hash (state) } Kind :: Dynamic (label) => { 1 . hash (state) ; label . dyn_hash (state) ; } } } }
    };
}

impl_128!();