// Generated macro for new (function)
macro_rules! Depcrate_stream_for_eachnew {
() => {
// Module: crate::stream::for_each
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F > (s : S , f : F) -> ForEach < S , F > where S : Stream , F : FnMut (S :: Item) -> Result < () , S :: Error > + Send + 'static { ForEach { stream : s , f : f , } }
};
}
