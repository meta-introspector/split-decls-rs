// Generated macro for new (function)
macro_rules! Depcrate_fut_stream_mapnew {
() => {
// Module: crate::fut::stream::map
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new < S , A , F , U > (stream : S , f : F) -> Map < S , F > where S : ActorStream < A > , A : Actor , F : FnMut (S :: Item , & mut A , & mut A :: Context) -> U , { Map { stream , f } }
};
}
