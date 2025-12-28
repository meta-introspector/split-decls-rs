macro_rules! deps {
    () => {
        SsrError!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl std :: fmt :: Display for SsrError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Parse error: {}" , self . 0) } }
    };
}

impl_97!();