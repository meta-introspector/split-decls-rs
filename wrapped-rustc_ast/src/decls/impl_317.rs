macro_rules! deps {
    () => {
        DiffMode!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl Display for DiffMode { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { DiffMode :: Error => write ! (f , "Error") , DiffMode :: Source => write ! (f , "Source") , DiffMode :: Forward => write ! (f , "Forward") , DiffMode :: Reverse => write ! (f , "Reverse") , } } }
    };
}

impl_317!();