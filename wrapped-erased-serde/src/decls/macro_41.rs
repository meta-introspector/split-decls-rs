macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_41 {
    () => {
        deps!();
        impl_deserializer_for_trait_object ! ({ mut } Box < dyn Deserializer <'de > + Send + Sync + '_ >) ;
    };
}

macro_41!()