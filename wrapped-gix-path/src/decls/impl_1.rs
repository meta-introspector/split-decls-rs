macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl std :: fmt :: Display for Utf8Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str ("Could not convert to UTF8 or from UTF8 due to ill-formed input") } }
    };
}

impl_1!()