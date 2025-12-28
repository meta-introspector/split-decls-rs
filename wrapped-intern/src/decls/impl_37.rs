macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > Hash for Interned < T > { fn hash < H : Hasher > (& self , state : & mut H) { state . write_usize (Arc :: as_ptr (& self . arc) as * const () as usize) } }
    };
}

impl_37!()