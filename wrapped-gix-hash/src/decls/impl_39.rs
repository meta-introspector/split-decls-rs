macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl std :: fmt :: Display for Kind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Kind :: Sha1 => f . write_str ("SHA1") , } } }
    };
}

impl_39!();