macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl fmt :: Debug for Hasher { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("crc32fast::Hasher") . finish () } }
    };
}

impl_21!();