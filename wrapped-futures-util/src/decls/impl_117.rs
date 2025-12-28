macro_rules! impl_117 {
    () => {
        impl < Fut1 , Fut2 > TryFlatten < Fut1 , Fut2 > { pub (crate) fn new (future : Fut1) -> Self { Self :: First { f : future } } }
    };
}

impl_117!()