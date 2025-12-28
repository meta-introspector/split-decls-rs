macro_rules! deps {
    () => {
        HasTokens!();
        LazyAttrTokenStream!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl < T : HasTokens > HasTokens for Box < T > { fn tokens (& self) -> Option < & LazyAttrTokenStream > { (* * self) . tokens () } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { (* * self) . tokens_mut () } }
    };
}

impl_250!();