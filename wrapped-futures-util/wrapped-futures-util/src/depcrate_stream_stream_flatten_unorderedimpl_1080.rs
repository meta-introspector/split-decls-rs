// Generated macro for impl_1080 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1080 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1080"}
// Dependencies: {}
impl < St > PollStreamFut < St > { # [doc = " Constructs new `PollStreamFut` using given `stream`."] fn new (stream : impl Into < Option < St > >) -> Self { Self { stream : stream . into () } } }
};
}
