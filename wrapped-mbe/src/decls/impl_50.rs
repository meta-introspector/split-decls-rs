macro_rules! deps {
    () => {
        ExpandErrorKind!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl fmt :: Display for ExpandErrorKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ExpandErrorKind :: NoMatchingRule => f . write_str ("no rule matches input tokens") , ExpandErrorKind :: UnexpectedToken => f . write_str ("unexpected token in input") , ExpandErrorKind :: BindingError (e) => f . write_str (e) , ExpandErrorKind :: UnresolvedBinding (binding) => { f . write_str ("could not find binding ") ? ; f . write_str (binding) } ExpandErrorKind :: LimitExceeded => f . write_str ("Expand exceed limit") , ExpandErrorKind :: LeftoverTokens => f . write_str ("leftover tokens") , } } }
    };
}

impl_50!()