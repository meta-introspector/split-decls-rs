macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T : ? Sized + Hash > Hash for Arc < T > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) } }
    };
}

impl_145!();