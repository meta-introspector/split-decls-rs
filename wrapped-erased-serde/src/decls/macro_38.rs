macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_38 {
    () => {
        deps!();
        impl_deserializer_for_trait_object ! ({ mut } Box < dyn Deserializer <'de > + '_ >) ;
    };
}

macro_38!();