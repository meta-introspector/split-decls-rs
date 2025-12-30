// Generated macro for with_position (function)
macro_rules! Depcrate_with_positionwith_position {
() => {
// Module: crate::with_position
// Provides: {"with_position"}
// Dependencies: {}
# [doc = " Create a new `WithPosition` iterator."] pub fn with_position < I > (iter : I) -> WithPosition < I > where I : Iterator , { WithPosition { handled_first : false , peekable : iter . fuse () . peekable () , } }
};
}
