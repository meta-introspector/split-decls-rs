macro_rules! deps {
    () => {
        CopiedConsumer!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        impl < C > CopiedConsumer < C > { fn new (base : C) -> Self { CopiedConsumer { base } } }
    };
}

impl_386!();