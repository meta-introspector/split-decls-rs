macro_rules! deps {
    () => {
        GlobError!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl std :: fmt :: Display for GlobError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { self . 0 . fmt (f) } }
    };
}

impl_5!()