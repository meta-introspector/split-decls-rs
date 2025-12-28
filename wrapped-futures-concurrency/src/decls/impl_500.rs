macro_rules! deps {
    () => {
        WakerVec!();
        Zip!();
        PollVec!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < S > Zip < S > where S : Stream , { pub (crate) fn new (streams : Vec < S >) -> Self { let len = streams . len () ; Self { len , streams , wakers : WakerVec :: new (len) , output : (0 .. len) . map (| _ | MaybeUninit :: uninit ()) . collect () , state : PollVec :: new_pending (len) , done : false , } } }
    };
}

impl_500!();