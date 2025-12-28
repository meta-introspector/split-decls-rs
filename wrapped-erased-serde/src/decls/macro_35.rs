macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_35 {
    () => {
        deps!();
        impl_deserializer_for_trait_object ! ({ } & mut (dyn Deserializer <'de > + Send + '_)) ;
    };
}

macro_35!();