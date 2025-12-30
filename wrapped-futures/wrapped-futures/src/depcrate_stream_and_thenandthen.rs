// Generated macro for AndThen (struct)
macro_rules! Depcrate_stream_and_thenAndThen {
() => {
// Module: crate::stream::and_then
// Provides: {"AndThen"}
// Dependencies: {}
# [doc = " A stream combinator which chains a computation onto values produced by a"] # [doc = " stream."] # [doc = ""] # [doc = " This structure is produced by the `Stream::and_then` method."] pub struct AndThen < S , F , U > where U : IntoFuture , { stream : S , future : Option < U :: Future > , f : F , }
};
}
