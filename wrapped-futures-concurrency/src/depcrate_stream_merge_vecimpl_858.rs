// Generated macro for impl_858 (impl)
macro_rules! Depcrate_stream_merge_vecimpl_858 {
() => {
// Module: crate::stream::merge::vec
// Provides: {"impl_858"}
// Dependencies: {}
impl < S > Merge < S > where S : Stream , { pub (crate) fn new (streams : Vec < S >) -> Self { let len = streams . len () ; Self { wakers : WakerVec :: new (len) , state : PollVec :: new_pending (len) , indexer : Indexer :: new (len) , streams , complete : 0 , done : false , } } }
};
}
