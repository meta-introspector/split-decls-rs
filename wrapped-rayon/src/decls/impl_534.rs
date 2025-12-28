macro_rules! deps {
    () => {
        FlattenConsumer!();
    };
}

macro_rules! impl_534 {
    () => {
        deps!();
        impl < C > FlattenConsumer < C > { fn new (base : C) -> Self { FlattenConsumer { base } } }
    };
}

impl_534!()