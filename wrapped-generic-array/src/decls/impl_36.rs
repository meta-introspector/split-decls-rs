macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T : Hash , N : ArrayLength > Hash for GenericArray < T , N > { # [inline] fn hash < H > (& self , state : & mut H) where H : Hasher , { Hash :: hash (self . as_slice () , state) } }
    };
}

impl_36!()