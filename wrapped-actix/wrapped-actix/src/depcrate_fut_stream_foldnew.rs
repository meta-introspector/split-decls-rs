// Generated macro for new (function)
macro_rules! Depcrate_fut_stream_foldnew {
() => {
// Module: crate::fut::stream::fold
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new < S , A , F , Fut > (stream : S , f : F , t : Fut :: Output) -> Fold < S , F , Fut , Fut :: Output > where S : ActorStream < A > , A : Actor , F : FnMut (Fut :: Output , S :: Item , & mut A , & mut A :: Context) -> Fut , Fut : ActorFuture < A > , { Fold { stream , f , accum : Some (t) , future : None , } }
};
}
