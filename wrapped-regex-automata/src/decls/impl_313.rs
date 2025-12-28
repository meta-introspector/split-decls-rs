macro_rules! deps {
    () => {
        RetryQuadraticError!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl core :: fmt :: Display for RetryQuadraticError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "regex engine gave up to avoid quadratic behavior") } }
    };
}

impl_313!()