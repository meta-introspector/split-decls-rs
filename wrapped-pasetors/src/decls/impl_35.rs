macro_rules! deps {
    () => {
        SymmetricKey!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < V > Debug for SymmetricKey < V > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "SymmetricKey {{***OMITTED***}}") } }
    };
}

impl_35!()