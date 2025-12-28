macro_rules! deps {
    () => {
        SharedState!();
    };
}

macro_rules! SerializationSinkBuilder {
    () => {
        deps!();
        pub struct SerializationSinkBuilder (SharedState) ;
    };
}

SerializationSinkBuilder!();