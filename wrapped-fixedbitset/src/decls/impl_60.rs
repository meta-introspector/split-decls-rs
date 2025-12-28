macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Hash for Block { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { Hash :: hash_slice (& self . into_usize_array () , hasher) ; } }
    };
}

impl_60!();