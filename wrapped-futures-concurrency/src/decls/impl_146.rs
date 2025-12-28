macro_rules! deps {
    () => {
        IntoConcurrentStream!();
        FromConcurrentStream!();
        ResultVecConsumer!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < T , E > FromConcurrentStream < Result < T , E > > for Result < Vec < T > , E > { async fn from_concurrent_stream < S > (iter : S) -> Self where S : IntoConcurrentStream < Item = Result < T , E > > , { let stream = iter . into_co_stream () ; let mut output = Ok (Vec :: with_capacity (stream . size_hint () . 1 . unwrap_or_default ())) ; stream . drive (ResultVecConsumer :: new (& mut output)) . await ; output } }
    };
}

impl_146!()