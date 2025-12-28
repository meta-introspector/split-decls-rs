macro_rules! deps {
    () => {
        PrefixEntryPoint!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl PrefixEntryPoint { pub fn parse (& self , input : & Input , edition : Edition) -> Output { let entry_point : fn (& '_ mut parser :: Parser < '_ >) = match self { PrefixEntryPoint :: Vis => grammar :: entry :: prefix :: vis , PrefixEntryPoint :: Block => grammar :: entry :: prefix :: block , PrefixEntryPoint :: Stmt => grammar :: entry :: prefix :: stmt , PrefixEntryPoint :: Pat => grammar :: entry :: prefix :: pat , PrefixEntryPoint :: PatTop => grammar :: entry :: prefix :: pat_top , PrefixEntryPoint :: Ty => grammar :: entry :: prefix :: ty , PrefixEntryPoint :: Expr => grammar :: entry :: prefix :: expr , PrefixEntryPoint :: Path => grammar :: entry :: prefix :: path , PrefixEntryPoint :: Item => grammar :: entry :: prefix :: item , PrefixEntryPoint :: MetaItem => grammar :: entry :: prefix :: meta_item , } ; let mut p = parser :: Parser :: new (input , edition) ; entry_point (& mut p) ; let events = p . finish () ; event :: process (events) } }
    };
}

impl_16!()