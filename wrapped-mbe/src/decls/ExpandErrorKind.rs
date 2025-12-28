macro_rules! ExpandErrorKind {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone , Hash)] pub enum ExpandErrorKind { BindingError (Box < Box < str > >) , UnresolvedBinding (Box < Box < str > >) , LeftoverTokens , LimitExceeded , NoMatchingRule , UnexpectedToken , }
    };
}

ExpandErrorKind!()