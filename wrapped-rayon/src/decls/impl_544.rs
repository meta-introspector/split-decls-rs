macro_rules! deps {
    () => {
        FlattenIterConsumer!();
    };
}

macro_rules! impl_544 {
    () => {
        deps!();
        impl < C > FlattenIterConsumer < C > { fn new (base : C) -> Self { FlattenIterConsumer { base } } }
    };
}

impl_544!();