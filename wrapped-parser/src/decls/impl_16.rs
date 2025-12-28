macro_rules! deps {
    () => {
        FrontmatterError!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl std :: fmt :: Display for FrontmatterError { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . message . fmt (fmt) } }
    };
}

impl_16!();