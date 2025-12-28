macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_918 {
    () => {
        deps!();
        impl < T > Hash for Ptr < '_ , T > { fn hash < H : hash :: Hasher > (& self , st : & mut H) { let ptr = (self . 0) as * const T ; ptr . hash (st) } }
    };
}

impl_918!()