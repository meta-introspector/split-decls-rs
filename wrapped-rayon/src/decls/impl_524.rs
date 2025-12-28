macro_rules! deps {
    () => {
        FlatMapIterConsumer!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl < 'f , C , F > FlatMapIterConsumer < 'f , C , F > { fn new (base : C , map_op : & 'f F) -> Self { FlatMapIterConsumer { base , map_op } } }
    };
}

impl_524!();