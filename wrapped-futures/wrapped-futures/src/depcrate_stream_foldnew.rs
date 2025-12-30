// Generated macro for new (function)
macro_rules! Depcrate_stream_foldnew {
() => {
// Module: crate::stream::fold
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F , Fut , T > (s : S , f : F , t : T) -> Fold < S , F , Fut , T > where S : Stream , F : FnMut (T , S :: Item) -> Fut + Send + 'static , Fut : IntoFuture < Item = T > , Fut :: Error : Into < S :: Error > , T : Send + 'static { Fold { stream : s , f : f , state : State :: Ready (t) , } }
};
}
