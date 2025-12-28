macro_rules! deps {
    () => {
        RuleType!();
        ErrorVariant!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < R : RuleType > fmt :: Display for ErrorVariant < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ErrorVariant :: ParsingError { .. } => write ! (f , "parsing error: {}" , self . message ()) , ErrorVariant :: CustomError { .. } => write ! (f , "{}" , self . message ()) , } } }
    };
}

impl_17!();