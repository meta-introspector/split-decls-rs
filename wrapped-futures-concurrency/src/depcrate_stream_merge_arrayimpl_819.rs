// Generated macro for impl_819 (impl)
macro_rules! Depcrate_stream_merge_arrayimpl_819 {
() => {
// Module: crate::stream::merge::array
// Provides: {"impl_819"}
// Dependencies: {}
impl < S , const N : usize > Merge < S , N > where S : Stream , { pub (crate) fn new (streams : [S ; N]) -> Self { Self { streams , indexer : Indexer :: new (N) , wakers : WakerArray :: new () , state : PollArray :: new_pending () , complete : 0 , done : false , } } }
};
}
