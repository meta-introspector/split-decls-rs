macro_rules! deps {
    () => {
        PollArray!();
        Zip!();
        WakerArray!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl < S , const N : usize > Zip < S , N > where S : Stream , { pub (crate) fn new (streams : [S ; N]) -> Self { Self { streams , output : array :: from_fn (| _ | MaybeUninit :: uninit ()) , state : PollArray :: new_pending () , wakers : WakerArray :: new () , done : false , } } }
    };
}

impl_476!()