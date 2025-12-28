macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl hash :: Hash for BytesMut { fn hash < H > (& self , state : & mut H) where H : hash :: Hasher , { let s : & [u8] = self . as_ref () ; s . hash (state) ; } }
    };
}

impl_198!();