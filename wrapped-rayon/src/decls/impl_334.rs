macro_rules! deps {
    () => {
        ClonedConsumer!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < C > ClonedConsumer < C > { fn new (base : C) -> Self { ClonedConsumer { base } } }
    };
}

impl_334!()