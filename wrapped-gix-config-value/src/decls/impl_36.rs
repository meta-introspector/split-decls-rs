macro_rules! deps {
    () => {
        Suffix!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Display for Suffix { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Kibi => write ! (f , "k") , Self :: Mebi => write ! (f , "m") , Self :: Gibi => write ! (f , "g") , } } }
    };
}

impl_36!();