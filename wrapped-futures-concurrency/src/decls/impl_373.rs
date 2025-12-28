macro_rules! deps {
    () => {
        OutputVec!();
        TryJoin!();
        PollVec!();
        FutureVec!();
        WakerVec!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl < Fut , T , E > TryJoin < Fut , T , E > where Fut : Future < Output = Result < T , E > > , { # [inline] pub (crate) fn new (futures : Vec < Fut >) -> Self { let len = futures . len () ; Self { consumed : false , pending : len , items : OutputVec :: uninit (len) , wakers : WakerVec :: new (len) , state : PollVec :: new_pending (len) , futures : FutureVec :: new (futures) , } } }
    };
}

impl_373!()