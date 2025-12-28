macro_rules! deps {
    () => {
        DefPathDataName!();
        DefPathData!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl fmt :: Display for DefPathData { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . name () { DefPathDataName :: Named (name) => f . write_str (name . as_str ()) , DefPathDataName :: Anon { namespace } => write ! (f , "{{{{{namespace}}}}}") , } } }
    };
}

impl_98!()