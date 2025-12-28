macro_rules! impl_39 {
    () => {
        impl < Fut1 , Fut2 > Flatten < Fut1 , Fut2 > { pub (crate) fn new (future : Fut1) -> Self { Self :: First { f : future } } }
    };
}

impl_39!()