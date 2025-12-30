// Generated macro for new (function)
macro_rules! Depcrate_stream_takenew {
() => {
// Module: crate::stream::take
// Provides: {"new"}
// Dependencies: {}
pub fn new < S > (s : S , amt : u64) -> Take < S > where S : Stream , { Take { stream : s , remaining : amt , } }
};
}
