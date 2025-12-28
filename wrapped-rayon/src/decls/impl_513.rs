macro_rules! deps {
    () => {
        FlatMapConsumer!();
    };
}

macro_rules! impl_513 {
    () => {
        deps!();
        impl < 'f , C , F > FlatMapConsumer < 'f , C , F > { fn new (base : C , map_op : & 'f F) -> Self { FlatMapConsumer { base , map_op } } }
    };
}

impl_513!();