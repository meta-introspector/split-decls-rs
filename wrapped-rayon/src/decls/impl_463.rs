macro_rules! deps {
    () => {
        FilterConsumer!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < 'p , C , P > FilterConsumer < 'p , C , P > { fn new (base : C , filter_op : & 'p P) -> Self { FilterConsumer { base , filter_op } } }
    };
}

impl_463!();