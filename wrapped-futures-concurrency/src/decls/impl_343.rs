macro_rules! deps {
    () => {
        PollArray!();
        FutureArray!();
        WakerArray!();
        TryJoin!();
        OutputArray!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < Fut , T , E , const N : usize > TryJoin < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > , { # [inline] pub (crate) fn new (futures : [Fut ; N]) -> Self { Self { consumed : false , pending : N , items : OutputArray :: uninit () , wakers : WakerArray :: new () , state : PollArray :: new_pending () , futures : FutureArray :: new (futures) , } } }
    };
}

impl_343!();