macro_rules! impl_768 {
    () => {
        impl < Fut > Once < Fut > { pub (crate) fn new (future : Fut) -> Self { Self { future : Some (future) } } }
    };
}

impl_768!();