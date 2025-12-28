macro_rules! deps {
    () => {
        AttrInput!();
        Display!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl fmt :: Display for AttrInput { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AttrInput :: Literal (lit) => write ! (f , " = {lit}") , AttrInput :: TokenTree (tt) => tt . fmt (f) , } } }
    };
}

impl_7!()