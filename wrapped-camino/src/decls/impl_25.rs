macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl fmt :: Display for Utf8PathBuf { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_25!();