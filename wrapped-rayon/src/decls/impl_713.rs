macro_rules! deps {
    () => {
        MapInitConsumer!();
    };
}

macro_rules! impl_713 {
    () => {
        deps!();
        impl < 'f , C , INIT , F > MapInitConsumer < 'f , C , INIT , F > { fn new (base : C , init : & 'f INIT , map_op : & 'f F) -> Self { MapInitConsumer { base , init , map_op } } }
    };
}

impl_713!();