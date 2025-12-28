macro_rules! deps {
    () => {
        Extern!();
        StrLit!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl Extern { pub fn from_abi (abi : Option < StrLit > , span : Span) -> Extern { match abi { Some (name) => Extern :: Explicit (name , span) , None => Extern :: Implicit (span) , } } pub fn span (self) -> Option < Span > { match self { Extern :: None => None , Extern :: Implicit (span) | Extern :: Explicit (_ , span) => Some (span) , } } }
    };
}

impl_210!();