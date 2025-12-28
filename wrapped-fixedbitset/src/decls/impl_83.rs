macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Hash for FixedBitSet { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . length . hash (state) ; self . as_simd_slice () . hash (state) ; } }
    };
}

impl_83!();