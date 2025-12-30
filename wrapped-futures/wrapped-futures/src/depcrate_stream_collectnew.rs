// Generated macro for new (function)
macro_rules! Depcrate_stream_collectnew {
() => {
// Module: crate::stream::collect
// Provides: {"new"}
// Dependencies: {}
pub fn new < S > (s : S) -> Collect < S > where S : Stream , { Collect { stream : s , items : Vec :: new () , } }
};
}
