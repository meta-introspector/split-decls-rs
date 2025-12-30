// Generated macro for new (function)
macro_rules! Depcrate_fut_stream_take_whilenew {
() => {
// Module: crate::fut::stream::take_while
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new < S , A , F , Fut > (stream : S , f : F) -> TakeWhile < S , S :: Item , F , Fut > where S : ActorStream < A > , A : Actor , F : FnMut (& S :: Item , & mut A , & mut A :: Context) -> Fut , Fut : ActorFuture < A , Output = bool > , { TakeWhile { stream , f , pending_fut : None , pending_item : None , done_taking : false , } }
};
}
