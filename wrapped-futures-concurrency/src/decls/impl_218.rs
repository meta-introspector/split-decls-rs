macro_rules! deps {
    () => {
        Join!();
        FutureArray!();
        PollArray!();
        OutputArray!();
        WakerArray!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < Fut , const N : usize > Join < Fut , N > where Fut : Future , { # [inline] pub (crate) fn new (futures : [Fut ; N]) -> Self { Join { consumed : false , pending : N , items : OutputArray :: uninit () , wakers : WakerArray :: new () , state : PollArray :: new_pending () , futures : FutureArray :: new (futures) , } } }
    };
}

impl_218!();