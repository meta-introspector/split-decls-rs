macro_rules! deps {
    () => {
        Stmt!();
        LazyAttrTokenStream!();
        HasTokens!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl HasTokens for Stmt { fn tokens (& self) -> Option < & LazyAttrTokenStream > { self . kind . tokens () } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { self . kind . tokens_mut () } }
    };
}

impl_252!();