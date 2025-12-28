macro_rules! deps {
    () => {
        AsymmetricSecretKey!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < V > Debug for AsymmetricSecretKey < V > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "AsymmetricSecretKey {{***OMITTED***}}") } }
    };
}

impl_40!()