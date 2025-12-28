macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T , const CAP : usize > Hash for ArrayVec < T , CAP > where T : Hash , { fn hash < H : Hasher > (& self , state : & mut H) { Hash :: hash (& * * self , state) } }
    };
}

impl_73!();