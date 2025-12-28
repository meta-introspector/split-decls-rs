macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
        HasTokens!();
    };
}

macro_rules! impl_has_tokens_none {
    () => {
        deps!();
        macro_rules ! impl_has_tokens_none { ($ ($ T : ty) ,+ $ (,) ?) => { $ (impl HasTokens for $ T { fn tokens (& self) -> Option <& LazyAttrTokenStream > { None } fn tokens_mut (& mut self) -> Option <& mut Option < LazyAttrTokenStream >> { None } }) + } ; }
    };
}

impl_has_tokens_none!()