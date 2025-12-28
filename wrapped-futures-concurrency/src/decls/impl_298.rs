macro_rules! deps {
    () => {
        RaceOk!();
        AggregateError!();
        PollArray!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < Fut , T , E , const N : usize > RaceOkTrait for [Fut ; N] where Fut : IntoFuture < Output = Result < T , E > > , { type Output = T ; type Error = AggregateError < E , N > ; type Future = RaceOk < Fut :: IntoFuture , T , E , N > ; fn race_ok (self) -> Self :: Future { RaceOk { futures : self . map (| fut | fut . into_future ()) , errors : array :: from_fn (| _ | MaybeUninit :: uninit ()) , error_states : PollArray :: new_pending () , completed : 0 , } } }
    };
}

impl_298!();