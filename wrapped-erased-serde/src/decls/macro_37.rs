macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_37 {
    () => {
        deps!();
        impl_deserializer_for_trait_object ! ({ } & mut (dyn Deserializer <'de > + Send + Sync + '_)) ;
    };
}

macro_37!();