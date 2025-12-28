macro_rules! impl_112 {
    () => {
        impl < Fut > IntoFuture < Fut > { # [inline] pub (crate) fn new (future : Fut) -> Self { Self { future } } }
    };
}

impl_112!()