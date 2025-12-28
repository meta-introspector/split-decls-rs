macro_rules! deps {
    () => {
        TryJoin!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < Fut , T , E , const N : usize > TryJoinTrait for [Fut ; N] where Fut : IntoFuture < Output = Result < T , E > > , { type Output = [T ; N] ; type Error = E ; type Future = TryJoin < Fut :: IntoFuture , T , E , N > ; fn try_join (self) -> Self :: Future { TryJoin :: new (self . map (IntoFuture :: into_future)) } }
    };
}

impl_344!()