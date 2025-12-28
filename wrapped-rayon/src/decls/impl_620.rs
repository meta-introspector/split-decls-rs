macro_rules! deps {
    () => {
        InspectConsumer!();
    };
}

macro_rules! impl_620 {
    () => {
        deps!();
        impl < 'f , C , F > InspectConsumer < 'f , C , F > { fn new (base : C , inspect_op : & 'f F) -> Self { InspectConsumer { base , inspect_op } } }
    };
}

impl_620!()