macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl std :: hash :: Hash for File { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . bytes . as_ptr () . hash (state) ; } }
    };
}

impl_435!();