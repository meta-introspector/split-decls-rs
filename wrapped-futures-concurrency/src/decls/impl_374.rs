macro_rules! deps {
    () => {
        TryJoin!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl < Fut , T , E > TryJoinTrait for Vec < Fut > where Fut : IntoFuture < Output = Result < T , E > > , { type Output = Vec < T > ; type Error = E ; type Future = TryJoin < Fut :: IntoFuture , T , E > ; fn try_join (self) -> Self :: Future { TryJoin :: new (self . into_iter () . map (IntoFuture :: into_future) . collect ()) } }
    };
}

impl_374!();