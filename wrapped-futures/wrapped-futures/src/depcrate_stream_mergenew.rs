// Generated macro for new (function)
macro_rules! Depcrate_stream_mergenew {
() => {
// Module: crate::stream::merge
// Provides: {"new"}
// Dependencies: {}
pub fn new < S1 , S2 > (stream1 : S1 , stream2 : S2) -> Merge < S1 , S2 > where S1 : Stream , S2 : Stream < Error = S1 :: Error > { Merge { stream1 : stream1 . fuse () , stream2 : stream2 . fuse () , queued_error : None , } }
};
}
