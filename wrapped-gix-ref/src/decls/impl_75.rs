macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl fmt :: Display for Target { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Target :: Object (oid) => oid . fmt (f) , Target :: Symbolic (name) => write ! (f , "ref: {}" , name . as_bstr ()) , } } }
    };
}

impl_75!();