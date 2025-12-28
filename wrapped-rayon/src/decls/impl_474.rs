macro_rules! deps {
    () => {
        FilterMapConsumer!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl < 'p , C , P : 'p > FilterMapConsumer < 'p , C , P > { fn new (base : C , filter_op : & 'p P) -> Self { FilterMapConsumer { base , filter_op } } }
    };
}

impl_474!()