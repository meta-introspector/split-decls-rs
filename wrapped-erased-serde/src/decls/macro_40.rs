macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_40 {
    () => {
        deps!();
        impl_deserializer_for_trait_object ! ({ mut } Box < dyn Deserializer <'de > + Sync + '_ >) ;
    };
}

macro_40!();