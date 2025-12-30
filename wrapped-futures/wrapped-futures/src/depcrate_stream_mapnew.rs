// Generated macro for new (function)
macro_rules! Depcrate_stream_mapnew {
() => {
// Module: crate::stream::map
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F , U > (s : S , f : F) -> Map < S , F > where S : Stream , F : FnMut (S :: Item) -> U + Send + 'static , U : Send + 'static , { Map { stream : s , f : f , } }
};
}
