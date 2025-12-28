macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T > Hash for Idx < T > { fn hash < H : Hasher > (& self , state : & mut H) { self . raw . hash (state) ; } }
    };
}

impl_33!();