macro_rules! deps {
    () => {
        Cfg!();
        Platform!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl fmt :: Display for Platform { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Platform :: Name (ref n) => n . fmt (f) , Platform :: Cfg (ref e) => write ! (f , "cfg({})" , e) , } } }
    };
}

impl_38!();