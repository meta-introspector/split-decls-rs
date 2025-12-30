// Generated macro for new (function)
macro_rules! Depcrate_fut_stream_thennew {
() => {
// Module: crate::fut::stream::then
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new < S , A , F , Fut > (stream : S , f : F) -> Then < S , F , Fut > where S : ActorStream < A > , A : Actor , F : FnMut (S :: Item , & mut A , & mut A :: Context) -> Fut , Fut : ActorFuture < A > , { Then { stream , f , future : None , } }
};
}
