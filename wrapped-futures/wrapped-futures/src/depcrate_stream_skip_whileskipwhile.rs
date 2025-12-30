// Generated macro for SkipWhile (struct)
macro_rules! Depcrate_stream_skip_whileSkipWhile {
() => {
// Module: crate::stream::skip_while
// Provides: {"SkipWhile"}
// Dependencies: {}
# [doc = " A stream combinator which skips elements of a stream while a predicate"] # [doc = " holds."] # [doc = ""] # [doc = " This structure is produced by the `Stream::skip_while` method."] pub struct SkipWhile < S , P , R > where S : Stream , R : IntoFuture { stream : S , pred : P , pending : Option < (R :: Future , S :: Item) > , done_skipping : bool , }
};
}
