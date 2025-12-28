macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match * self { Error :: Syntax (ref err) => err . fmt (f) , Error :: CompiledTooBig (limit) => write ! (f , "Compiled regex exceeds size limit of {limit} bytes." ,) , } } }
    };
}

impl_13!()