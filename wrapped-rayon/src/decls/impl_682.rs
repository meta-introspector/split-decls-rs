macro_rules! deps {
    () => {
        MapConsumer!();
    };
}

macro_rules! impl_682 {
    () => {
        deps!();
        impl < 'f , C , F > MapConsumer < 'f , C , F > { fn new (base : C , map_op : & 'f F) -> Self { MapConsumer { base , map_op } } }
    };
}

impl_682!()