// Generated macro for StreamFuture (struct)
macro_rules! Depcrate_stream_futureStreamFuture {
() => {
// Module: crate::stream::future
// Provides: {"StreamFuture"}
// Dependencies: {}
# [doc = " A combinator used to temporarily convert a stream into a future."] # [doc = ""] # [doc = " This future is returned by the `Stream::into_future` method."] pub struct StreamFuture < S > { stream : Option < S > , }
};
}
