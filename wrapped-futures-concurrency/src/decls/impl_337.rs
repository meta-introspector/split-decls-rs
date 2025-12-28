macro_rules! deps {
    () => {
        AggregateError!();
        MaybeDone!();
        RaceOk!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < Fut , T , E > RaceOkTrait for Vec < Fut > where Fut : IntoFuture < Output = Result < T , E > > , { type Output = T ; type Error = AggregateError < E > ; type Future = RaceOk < Fut :: IntoFuture , T , E > ; fn race_ok (self) -> Self :: Future { let elems : Box < [_] > = self . into_iter () . map (| fut | MaybeDone :: new (fut . into_future ())) . collect () ; RaceOk { elems : elems . into () , } } }
    };
}

impl_337!();