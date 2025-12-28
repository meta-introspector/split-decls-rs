macro_rules! deps {
    () => {
        OutputVec!();
        WakerVec!();
        PollVec!();
        FutureVec!();
        Join!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < Fut > Join < Fut > where Fut : Future , { pub (crate) fn new (futures : Vec < Fut >) -> Self { let len = futures . len () ; Join { consumed : false , pending : len , items : OutputVec :: uninit (len) , wakers : WakerVec :: new (len) , state : PollVec :: new_pending (len) , futures : FutureVec :: new (futures) , } } }
    };
}

impl_248!()