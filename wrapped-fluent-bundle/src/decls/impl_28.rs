macro_rules! deps {
    () => {
        ResolverError!();
        FluentError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl std :: fmt :: Display for FluentError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Overriding { kind , id } => { write ! (f , "Attempt to override an existing {}: \"{}\"." , kind , id) } Self :: ParserError (err) => write ! (f , "Parser error: {}" , err) , Self :: ResolverError (err) => write ! (f , "Resolver error: {}" , err) , } } }
    };
}

impl_28!()