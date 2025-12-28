macro_rules! deps {
    () => {
        Merge!();
        WakerVec!();
        Indexer!();
        PollVec!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < S > Merge < S > where S : Stream , { pub (crate) fn new (streams : Vec < S >) -> Self { let len = streams . len () ; Self { wakers : WakerVec :: new (len) , state : PollVec :: new_pending (len) , indexer : Indexer :: new (len) , streams , complete : 0 , done : false , } } }
    };
}

impl_459!();