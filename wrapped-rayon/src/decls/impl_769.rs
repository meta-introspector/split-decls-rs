macro_rules! deps {
    () => {
        PositionsConsumer!();
    };
}

macro_rules! impl_769 {
    () => {
        deps!();
        impl < 'p , C , P > PositionsConsumer < 'p , C , P > { fn new (base : C , predicate : & 'p P , offset : usize) -> Self { PositionsConsumer { base , predicate , offset , } } }
    };
}

impl_769!();