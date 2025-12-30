// Generated macro for new (function)
macro_rules! Depcrate_stream_thennew {
() => {
// Module: crate::stream::then
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F , U > (s : S , f : F) -> Then < S , F , U > where S : Stream , F : FnMut (Result < S :: Item , S :: Error >) -> U + Send + 'static , U : IntoFuture , { Then { stream : s , future : None , f : f , } }
};
}
