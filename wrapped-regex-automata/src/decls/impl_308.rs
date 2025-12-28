macro_rules! deps {
    () => {
        RetryError!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl core :: fmt :: Display for RetryError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match * self { RetryError :: Quadratic (ref err) => err . fmt (f) , RetryError :: Fail (ref err) => err . fmt (f) , } } }
    };
}

impl_308!()