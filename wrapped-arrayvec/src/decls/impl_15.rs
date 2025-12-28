macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < const CAP : usize > Hash for ArrayString < CAP > { fn hash < H : Hasher > (& self , h : & mut H) { (* * self) . hash (h) } }
    };
}

impl_15!();