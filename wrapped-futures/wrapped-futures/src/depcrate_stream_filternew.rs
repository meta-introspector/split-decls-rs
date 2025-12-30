// Generated macro for new (function)
macro_rules! Depcrate_stream_filternew {
() => {
// Module: crate::stream::filter
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F > (s : S , f : F) -> Filter < S , F > where S : Stream , F : FnMut (& S :: Item) -> bool + Send + 'static , { Filter { stream : s , f : f , } }
};
}
