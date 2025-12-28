macro_rules! deps {
    () => {
        Result!();
        MatchesError!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl std :: fmt :: Display for MatchesError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Downcast { actual , expected } => { writeln ! (f , "Could not downcast to {expected:?}, need to downcast to {actual:?}") } Self :: UnknownArgument { } => { writeln ! (f , "Unknown argument or group id.  Make sure you are using the argument id and not the short or long flags") } } } }
    };
}

impl_435!();