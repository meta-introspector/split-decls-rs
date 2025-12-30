// Generated macro for wrap_stream (function)
macro_rules! Depcrate_fut_streamwrap_stream {
() => {
// Module: crate::fut::stream
// Provides: {"wrap_stream"}
// Dependencies: {}
# [doc = " Converts normal stream into `ActorStream`"] pub fn wrap_stream < S , A > (stream : S) -> StreamWrap < S , A > where S : Stream , A : Actor , { StreamWrap { stream , _act : PhantomData , } }
};
}
