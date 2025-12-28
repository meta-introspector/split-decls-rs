macro_rules! deps {
    () => {
        CppStruct!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl std :: hash :: Hash for CppStruct { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . def . hash (state) ; } }
    };
}

impl_290!()