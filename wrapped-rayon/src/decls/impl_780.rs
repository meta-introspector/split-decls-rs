macro_rules! deps {
    () => {
        Reducer!();
        UnindexedConsumer!();
        ProductConsumer!();
    };
}

macro_rules! impl_780 {
    () => {
        deps!();
        impl < P , T > UnindexedConsumer < T > for ProductConsumer < P > where P : Send + Product < T > + Product , { fn split_off_left (& self) -> Self { ProductConsumer :: new () } fn to_reducer (& self) -> Self :: Reducer { ProductConsumer :: new () } }
    };
}

impl_780!()