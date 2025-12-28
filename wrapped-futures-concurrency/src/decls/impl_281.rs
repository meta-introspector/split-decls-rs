macro_rules! deps {
    () => {
        Race!();
        Indexer!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl < Fut > RaceTrait for Vec < Fut > where Fut : IntoFuture , { type Output = Fut :: Output ; type Future = Race < Fut :: IntoFuture > ; fn race (self) -> Self :: Future { Race { indexer : Indexer :: new (self . len ()) , futures : self . into_iter () . map (| fut | fut . into_future ()) . collect () , done : false , } } }
    };
}

impl_281!();