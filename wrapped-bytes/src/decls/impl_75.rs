macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl hash :: Hash for Bytes { fn hash < H > (& self , state : & mut H) where H : hash :: Hasher , { self . as_slice () . hash (state) ; } }
    };
}

impl_75!();