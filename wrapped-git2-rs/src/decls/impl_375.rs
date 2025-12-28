macro_rules! deps {
    () => {
        Error!();
        ErrorClass!();
        ErrorCode!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . message) ? ; match self . class () { ErrorClass :: None => { } other => write ! (f , "; class={:?} ({})" , other , self . klass) ? , } match self . code () { ErrorCode :: GenericError => { } other => write ! (f , "; code={:?} ({})" , other , self . code) ? , } Ok (()) } }
    };
}

impl_375!();