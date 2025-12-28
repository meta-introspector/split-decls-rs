macro_rules! deps {
    () => {
        ExtractError!();
        Notification!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl fmt :: Display for ExtractError < Notification > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ExtractError :: MethodMismatch (req) => { write ! (f , "Method mismatch for notification '{}'" , req . method) } ExtractError :: JsonError { method , error } => { write ! (f , "Invalid notification\nMethod: {method}\n error: {error}") } } } }
    };
}

impl_8!()