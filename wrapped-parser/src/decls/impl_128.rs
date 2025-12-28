macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < T : Hash > Hash for Positioned < T > { fn hash < H : Hasher > (& self , state : & mut H) { self . node . hash (state) } }
    };
}

impl_128!();