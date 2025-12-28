macro_rules! deps {
    () => {
        WakerArray!();
        Indexer!();
        Merge!();
        PollArray!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < S , const N : usize > Merge < S , N > where S : Stream , { pub (crate) fn new (streams : [S ; N]) -> Self { Self { streams , indexer : Indexer :: new (N) , wakers : WakerArray :: new () , state : PollArray :: new_pending () , complete : 0 , done : false , } } }
    };
}

impl_435!()