// Generated macro for block_on_stream (function)
macro_rules! Depcrate_local_poolblock_on_stream {
() => {
// Module: crate::local_pool
// Provides: {"block_on_stream"}
// Dependencies: {}
# [doc = " Turn a stream into a blocking iterator."] # [doc = ""] # [doc = " When `next` is called on the resulting `BlockingStream`, the caller"] # [doc = " will be blocked until the next element of the `Stream` becomes available."] pub fn block_on_stream < S : Stream + Unpin > (stream : S) -> BlockingStream < S > { BlockingStream { stream } }
};
}
