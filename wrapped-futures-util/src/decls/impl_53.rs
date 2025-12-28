macro_rules! impl_53 {
    () => {
        impl < Fut , F > Map < Fut , F > { # [doc = " Creates a new Map."] pub (crate) fn new (future : Fut , f : F) -> Self { Self :: Incomplete { future , f } } }
    };
}

impl_53!()