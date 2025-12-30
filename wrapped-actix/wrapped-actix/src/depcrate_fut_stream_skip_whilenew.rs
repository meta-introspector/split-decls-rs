// Generated macro for new (function)
macro_rules! Depcrate_fut_stream_skip_whilenew {
() => {
// Module: crate::fut::stream::skip_while
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new < S , A , F , Fut > (stream : S , f : F) -> SkipWhile < S , S :: Item , F , Fut > where S : ActorStream < A > , A : Actor , F : FnMut (& S :: Item , & mut A , & mut A :: Context) -> Fut , Fut : ActorFuture < A , Output = bool > , { SkipWhile { stream , f , pending_fut : None , pending_item : None , done_skipping : false , } }
};
}
