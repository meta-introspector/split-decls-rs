macro_rules! deps {
    () => {
        Join!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < Fut , const N : usize > JoinTrait for [Fut ; N] where Fut : IntoFuture , { type Output = [Fut :: Output ; N] ; type Future = Join < Fut :: IntoFuture , N > ; # [inline] fn join (self) -> Self :: Future { Join :: new (self . map (IntoFuture :: into_future)) } }
    };
}

impl_219!();