macro_rules! deps {
    () => {
        Request!();
        ExtractError!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl fmt :: Display for ExtractError < Request > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ExtractError :: MethodMismatch (req) => { write ! (f , "Method mismatch for request '{}'" , req . method) } ExtractError :: JsonError { method , error } => { write ! (f , "Invalid request\nMethod: {method}\n error: {error}" ,) } } } }
    };
}

impl_6!()