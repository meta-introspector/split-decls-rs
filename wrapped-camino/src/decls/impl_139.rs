macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl Hash for Utf8Path { fn hash < H : Hasher > (& self , state : & mut H) { for component in self . components () { component . hash (state) } } }
    };
}

impl_139!();