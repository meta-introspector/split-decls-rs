// Generated macro for OrElse (struct)
macro_rules! Depcrate_stream_or_elseOrElse {
() => {
// Module: crate::stream::or_else
// Provides: {"OrElse"}
// Dependencies: {}
# [doc = " A stream combinator which chains a computation onto errors produced by a"] # [doc = " stream."] # [doc = ""] # [doc = " This structure is produced by the `Stream::or_else` method."] pub struct OrElse < S , F , U > where U : IntoFuture , { stream : S , future : Option < U :: Future > , f : F , }
};
}
