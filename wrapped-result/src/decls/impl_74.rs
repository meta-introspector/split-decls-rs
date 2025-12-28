macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl core :: hash :: Hash for Error { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . code . hash (state) ; } }
    };
}

impl_74!()