macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_34 {
    () => {
        deps!();
        impl_deserializer_for_trait_object ! ({ } & mut (dyn Deserializer <'de > + '_)) ;
    };
}

macro_34!()