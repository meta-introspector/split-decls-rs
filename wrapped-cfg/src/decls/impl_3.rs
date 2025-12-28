macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl fmt :: Display for CfgAtom { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CfgAtom :: Flag (name) => name . fmt (f) , CfgAtom :: KeyValue { key , value } => write ! (f , "{key} = {value:?}") , } } }
    };
}

impl_3!();