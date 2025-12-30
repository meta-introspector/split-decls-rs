// Generated macro for Then (struct)
macro_rules! Depcrate_stream_thenThen {
() => {
// Module: crate::stream::then
// Provides: {"Then"}
// Dependencies: {}
# [doc = " A stream combinator which chains a computation onto each item produced by a"] # [doc = " stream."] # [doc = ""] # [doc = " This structure is produced by the `Stream::then` method."] pub struct Then < S , F , U > where U : IntoFuture , { stream : S , future : Option < U :: Future > , f : F , }
};
}
