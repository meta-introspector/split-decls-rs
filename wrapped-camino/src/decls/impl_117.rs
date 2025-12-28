macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl Hash for Utf8PathBuf { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_path () . hash (state) } }
    };
}

impl_117!()