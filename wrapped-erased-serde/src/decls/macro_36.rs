macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_36 {
    () => {
        deps!();
        impl_deserializer_for_trait_object ! ({ } & mut (dyn Deserializer <'de > + Sync + '_)) ;
    };
}

macro_36!();