macro_rules! deps {
    () => {
        ParseNode!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl Display for ParseNode { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: SectionHeader => write ! (f , "section header") , Self :: Name => write ! (f , "name") , Self :: Value => write ! (f , "value") , } } }
    };
}

impl_152!()