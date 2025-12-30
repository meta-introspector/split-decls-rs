// Generated macro for new (function)
macro_rules! Depcrate_stream_and_thennew {
() => {
// Module: crate::stream::and_then
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F , U > (s : S , f : F) -> AndThen < S , F , U > where S : Stream , F : FnMut (S :: Item) -> U + Send + 'static , U : IntoFuture < Error = S :: Error > , { AndThen { stream : s , future : None , f : f , } }
};
}
