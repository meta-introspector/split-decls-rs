macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl fmt :: Debug for Utf8PathBuf { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_7!()