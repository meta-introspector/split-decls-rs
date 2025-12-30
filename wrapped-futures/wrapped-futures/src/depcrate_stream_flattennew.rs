// Generated macro for new (function)
macro_rules! Depcrate_stream_flattennew {
() => {
// Module: crate::stream::flatten
// Provides: {"new"}
// Dependencies: {}
pub fn new < S > (s : S) -> Flatten < S > where S : Stream , S :: Item : Stream , < S :: Item as Stream > :: Error : From < S :: Error > , { Flatten { stream : s , next : None , } }
};
}
