macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_39 {
    () => {
        deps!();
        impl_deserializer_for_trait_object ! ({ mut } Box < dyn Deserializer <'de > + Send + '_ >) ;
    };
}

macro_39!()