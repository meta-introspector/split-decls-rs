macro_rules! deps {
    () => {
        MapWithConsumer!();
    };
}

macro_rules! impl_700 {
    () => {
        deps!();
        impl < 'f , C , U , F > MapWithConsumer < 'f , C , U , F > { fn new (base : C , item : U , map_op : & 'f F) -> Self { MapWithConsumer { base , item , map_op } } }
    };
}

impl_700!()