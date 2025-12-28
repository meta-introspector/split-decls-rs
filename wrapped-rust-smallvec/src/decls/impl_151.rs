macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < T : Hash , const N : usize > Hash for SmallVec < T , N > { fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
    };
}

impl_151!()