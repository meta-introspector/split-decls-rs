macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] impl < E : fmt :: Display , const N : usize > fmt :: Display for AggregateError < E , N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} errors occured" , self . inner . len ()) } }
    };
}

impl_306!()