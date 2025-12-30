// Generated macro for empty (function)
macro_rules! Depcrate_stream_emptyempty {
() => {
// Module: crate::stream::empty
// Provides: {"empty"}
// Dependencies: {}
# [doc = " Creates a stream which contains no elements."] # [doc = ""] # [doc = " The returned stream will always return `Ready(None)` when polled."] pub fn empty < T > () -> Empty < T > { assert_stream :: < T , _ > (Empty { _phantom : PhantomData }) }
};
}
