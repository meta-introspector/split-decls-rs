macro_rules! deps {
    () => {
        ConstContext!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        # [doc = " A colloquial, trivially pluralizable description of this const context for use in error"] # [doc = " messages."] impl fmt :: Display for ConstContext { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Const { .. } => write ! (f , "constant") , Self :: Static (_) => write ! (f , "static") , Self :: ConstFn => write ! (f , "constant function") , } } }
    };
}

impl_215!()