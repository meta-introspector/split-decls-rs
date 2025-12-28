macro_rules! deps {
    () => {
        SumConsumer!();
    };
}

macro_rules! impl_864 {
    () => {
        deps!();
        impl < S : Send > SumConsumer < S > { fn new () -> SumConsumer < S > { SumConsumer { _marker : PhantomData , } } }
    };
}

impl_864!();