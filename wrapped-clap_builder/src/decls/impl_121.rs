macro_rules! deps {
    () => {
        Result!();
        OsStr!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl std :: fmt :: Debug for OsStr { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Debug :: fmt (self . as_os_str () , f) } }
    };
}

impl_121!()