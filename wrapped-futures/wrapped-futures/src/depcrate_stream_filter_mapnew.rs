// Generated macro for new (function)
macro_rules! Depcrate_stream_filter_mapnew {
() => {
// Module: crate::stream::filter_map
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F , B > (s : S , f : F) -> FilterMap < S , F > where S : Stream , F : FnMut (S :: Item) -> Option < B > + Send + 'static , { FilterMap { stream : s , f : f , } }
};
}
