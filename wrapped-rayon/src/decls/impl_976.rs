macro_rules! deps {
    () => {
        UpdateConsumer!();
    };
}

macro_rules! impl_976 {
    () => {
        deps!();
        impl < 'f , C , F > UpdateConsumer < 'f , C , F > { fn new (base : C , update_op : & 'f F) -> Self { UpdateConsumer { base , update_op } } }
    };
}

impl_976!()