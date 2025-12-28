macro_rules! deps {
    () => {
        Cfg!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl fmt :: Display for Cfg { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Cfg :: Name (ref s) => s . fmt (f) , Cfg :: KeyPair (ref k , ref v) => write ! (f , "{} = \"{}\"" , k , v) , } } }
    };
}

impl_14!();