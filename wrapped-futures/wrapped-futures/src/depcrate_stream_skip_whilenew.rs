// Generated macro for new (function)
macro_rules! Depcrate_stream_skip_whilenew {
() => {
// Module: crate::stream::skip_while
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , P , R > (s : S , p : P) -> SkipWhile < S , P , R > where S : Stream , P : FnMut (& S :: Item) -> R + Send + 'static , R : IntoFuture < Item = bool , Error = S :: Error > , { SkipWhile { stream : s , pred : p , pending : None , done_skipping : false , } }
};
}
