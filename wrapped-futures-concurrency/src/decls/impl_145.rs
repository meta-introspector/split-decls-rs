macro_rules! deps {
    () => {
        VecConsumer!();
        FromConcurrentStream!();
        IntoConcurrentStream!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T > FromConcurrentStream < T > for Vec < T > { async fn from_concurrent_stream < S > (iter : S) -> Self where S : IntoConcurrentStream < Item = T > , { let stream = iter . into_co_stream () ; let mut output = Vec :: with_capacity (stream . size_hint () . 1 . unwrap_or_default ()) ; stream . drive (VecConsumer :: new (& mut output)) . await ; output } }
    };
}

impl_145!();