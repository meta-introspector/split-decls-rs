// Generated macro for new (function)
macro_rules! Depcrate_stream_skipnew {
() => {
// Module: crate::stream::skip
// Provides: {"new"}
// Dependencies: {}
pub fn new < S > (s : S , amt : u64) -> Skip < S > where S : Stream , { Skip { stream : s , remaining : amt , } }
};
}
