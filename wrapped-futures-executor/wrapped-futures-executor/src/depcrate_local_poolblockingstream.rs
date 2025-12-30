// Generated macro for BlockingStream (struct)
macro_rules! Depcrate_local_poolBlockingStream {
() => {
// Module: crate::local_pool
// Provides: {"BlockingStream"}
// Dependencies: {}
# [doc = " An iterator which blocks on values from a stream until they become available."] # [derive (Debug)] pub struct BlockingStream < S : Stream + Unpin > { stream : S , }
};
}
