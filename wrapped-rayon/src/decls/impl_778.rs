macro_rules! deps {
    () => {
        ProductConsumer!();
    };
}

macro_rules! impl_778 {
    () => {
        deps!();
        impl < P : Send > ProductConsumer < P > { fn new () -> ProductConsumer < P > { ProductConsumer { _marker : PhantomData , } } }
    };
}

impl_778!()