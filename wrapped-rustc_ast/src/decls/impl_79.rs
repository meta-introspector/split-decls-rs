macro_rules! deps {
    () => {
        Block!();
        Expr!();
        LocalKind!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl LocalKind { pub fn init (& self) -> Option < & Expr > { match self { Self :: Decl => None , Self :: Init (i) | Self :: InitElse (i , _) => Some (i) , } } pub fn init_else_opt (& self) -> Option < (& Expr , Option < & Block >) > { match self { Self :: Decl => None , Self :: Init (init) => Some ((init , None)) , Self :: InitElse (init , els) => Some ((init , Some (els))) , } } }
    };
}

impl_79!()