macro_rules! impl_125 {
    () => {
        impl < Fut1 , Fut2 > TryFlattenErr < Fut1 , Fut2 > { pub (crate) fn new (future : Fut1) -> Self { Self :: First { f : future } } }
    };
}

impl_125!()