macro_rules! deps {
    () => {
        BuildError!();
        BuildErrorKind!();
        NFA!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl core :: fmt :: Display for BuildError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self . kind { BuildErrorKind :: NFA (_) => write ! (f , "error building NFA") , BuildErrorKind :: InsufficientCacheCapacity { minimum , given } => { write ! (f , "given cache capacity ({given}) is smaller than \
                     minimum required ({minimum})" ,) } BuildErrorKind :: InsufficientStateIDCapacity { ref err } => { err . fmt (f) } BuildErrorKind :: Unsupported (ref msg) => { write ! (f , "unsupported regex feature for DFAs: {msg}") } } } }
    };
}

impl_256!();