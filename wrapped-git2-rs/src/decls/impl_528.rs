macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        impl Hash for Oid { fn hash < H : Hasher > (& self , into : & mut H) { self . raw . id . hash (into) } }
    };
}

impl_528!();