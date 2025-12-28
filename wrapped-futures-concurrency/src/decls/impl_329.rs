macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < E : fmt :: Display > fmt :: Display for AggregateError < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} errors occurred" , self . inner . len ()) } }
    };
}

impl_329!();