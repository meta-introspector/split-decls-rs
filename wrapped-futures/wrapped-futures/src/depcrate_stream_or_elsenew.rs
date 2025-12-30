// Generated macro for new (function)
macro_rules! Depcrate_stream_or_elsenew {
() => {
// Module: crate::stream::or_else
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F , U > (s : S , f : F) -> OrElse < S , F , U > where S : Stream , F : FnMut (S :: Error) -> U + Send + 'static , U : IntoFuture < Item = S :: Item > , { OrElse { stream : s , future : None , f : f , } }
};
}
