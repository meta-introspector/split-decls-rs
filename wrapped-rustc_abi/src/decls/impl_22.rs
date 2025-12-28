macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Hash for ExternAbi { fn hash < H : Hasher > (& self , state : & mut H) { self . as_str () . hash (state) ; u32 :: from_be_bytes (* b"ABI\0") . hash (state) ; } }
    };
}

impl_22!()