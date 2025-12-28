macro_rules! deps {
    () => {
        Race!();
        Indexer!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < Fut , const N : usize > RaceTrait for [Fut ; N] where Fut : IntoFuture , { type Output = Fut :: Output ; type Future = Race < Fut :: IntoFuture , N > ; fn race (self) -> Self :: Future { Race { futures : self . map (| fut | fut . into_future ()) , indexer : Indexer :: new (N) , done : false , } } }
    };
}

impl_260!()